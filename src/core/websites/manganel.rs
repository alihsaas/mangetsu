use futures::StreamExt;
use regex::Regex;
use reqwest::Url;
use scraper::{Html, Selector};

use crate::core::{
    connector::{FutureResult, StreamResult},
    error::Error,
    Chapter, Connector, ConnectorInfo, Connectors, GlobalAPI, Manga, Page,
};

#[derive(Debug, Clone)]
pub struct Manganel {
    info: ConnectorInfo,
}

impl Manganel {
    pub fn new() -> Self {
        Self {
			info: ConnectorInfo {
				id: "manganel",
				label: "Manganato",
				tags: vec!["manga", "webtoon", "english"],
				url: Url::parse("https://manganato.com").unwrap(),

				path: "/genre-all/",
		        manga_title_filter: Regex::new(r"(?i)(\s+manga|\s+webtoon|\s+others)+\s*$").unwrap(),
		        chapter_title_filter: Regex::new(r"(?i)^\s*(\\s+manga|\\s+webtoon|\\s+others)+").unwrap(),
		        query_manga_title: Selector::parse("div.container-main div.panel-story-info div.story-info-right h1").unwrap(),
		        query_mangas_page_count: Selector::parse("div.panel-page-number div.group-page a.page-last:last-of-type").unwrap(),
		        query_mangas: Selector::parse("div.genres-item-info h3 a.genres-item-name").unwrap(),

                query_icon: Selector::parse(".info-image > img:nth-child(1)").unwrap(),

		        query_chapters: Selector::parse(&[
		            "ul.row-content-chapter li a.chapter-name", // manganato, mangabat
		            "div.chapter_list ul li a", // mangairo
		            "div.chapter-list div.row span a", // mangakakalot(s), kissmangawebsite, manganeloinfo
		            "div.content.mCustomScrollbar div.chapter-list ul li.row div.chapter h4 a.xanh" // MangaPark
		        ].join(", ")).unwrap(),

		        query_pages: Selector::parse(&[
		            "div.container-chapter-reader img", // manganato, mangabat
		            "div.chapter-content div.panel-read-story img", // mangairo
		            "div#vungdoc img, div.vung-doc img, div.vung_doc img" // mangakakalot(s), kissmangawebsite, manganeloinfo
		        ].join(", ")).unwrap(),
			},
        }
    }
}

impl Connector for Manganel {
    fn get_connector_info(&self) -> ConnectorInfo {
        self.info.clone()
    }

    fn can_handle_uri(&self, uri: Url) -> bool {
        if let Some(domain) = uri.domain() {
            regex::Regex::new(r"^(m\.|chap\.)?(read)?manganato\.com$")
                .unwrap()
                .is_match(domain)
        } else {
            false
        }
    }

    fn get_manga_from_url(&self, manga_url: Url) -> FutureResult<Manga> {
        Box::pin(async move {
            let title = {
                let request = GlobalAPI::global()
                    .client
                    .get(manga_url.clone())
                    .send()
                    .await
                    .map_err(|err| Error::RequestFail(err.to_string()))?;

                let dom = Html::parse_document(&request.text().await.unwrap());
                let title = dom.select(&self.info.query_manga_title);
                title.last().unwrap().inner_html()
            };

            Ok(Manga {
                title,
                url: manga_url.clone(),
                icon_url: self.get_manga_icon(manga_url).await?,
                connector: Connectors::Manganel,
            })
        })
    }

    fn get_mangas(&self) -> StreamResult<Manga> {
        Box::pin(async_stream::try_stream! {
            let uri = self
                .info
                .url
                .join(&format!("{}{}", self.info.path, "1"))
                .expect("Malformed Url");
            let request = GlobalAPI::global()
                .client
                .get(uri)
                .send()
                .await
                .map_err(|err| Error::RequestFail(err.to_string()))?;

            let page_count = {
                let dom = Html::parse_document(&request.text().await.expect("Failed to decode response body."));
                let data = dom.select(&self.info.query_mangas_page_count);

                let element = data.last().unwrap().value();

                element
                    .attr("href")
                    .unwrap()
                    .matches(char::is_numeric)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            };

            for page in 1..page_count {
                let mut stream = self.get_mangas_from_page(page);
                while let Some(result) = stream.next().await {
                    let manga = result?;
                    yield manga;
                }
            };
        })
    }

    fn get_manga_icon(&self, manga_url: Url) -> FutureResult<Url> {
        Box::pin(async move {
            let src = {
                let request = GlobalAPI::global()
                    .client
                    .get(manga_url.clone())
                    .send()
                    .await
                    .map_err(|err| Error::RequestFail(err.to_string()))?;
                let dom = Html::parse_document(&request.text().await.unwrap());
                let data = dom.select(&self.info.query_icon);
                let element = data.last().unwrap().value();
                element.attr("src").unwrap().to_string()
            };
            Ok(Url::parse(&src).unwrap())
        })
    }

    fn get_mangas_from_page(&self, page: i32) -> StreamResult<Manga> {
        Box::pin(async_stream::try_stream! {
            let data: Vec<(String, Url)> = {
                let uri = self
                    .info
                    .url
                    .join(&format!("{}{}", self.info.path, page))
                    .expect("Malformed Url");
                let request = GlobalAPI::global()
                    .client
                    .get(uri)
                    .send()
                    .await
                    .map_err(|err| Error::RequestFail(err.to_string()))?;
                let dom = Html::parse_document(&request.text().await.unwrap());

                dom.select(&self.info.query_mangas)
                    .map(|element_ref| {
                        let element = element_ref.value();
                        let title = self
                            .info
                            .manga_title_filter
                            .replace(element.attr("title").unwrap(), "")
                            .trim()
                            .to_string();
                        let manga_url = Url::parse(element.attr("href").unwrap()).unwrap();
                        (title, manga_url)
                })
                .collect()
            };

            for info in data {
                let manga_url = info.1.clone();
                yield Manga {
                    title: info.0.clone(),
                    url: info.1.clone(),
                    icon_url: self.get_manga_icon(manga_url).await.unwrap(),
                    connector: Connectors::Manganel,
                }
            }
        })
    }

    fn get_chapters(&self, manga: Manga) -> StreamResult<Chapter> {
        Box::pin(async_stream::try_stream! {
            let info: Vec<(Url, String)> = {
                let request = GlobalAPI::global()
                    .client
                    .get(manga.url)
                    .send()
                    .await
                    .map_err(|err| Error::RequestFail(err.to_string()))?;
                let dom = Html::parse_document(&request.text().await.unwrap());
                dom.select(&self.info.query_chapters)
                    .map(|element_ref| {
                        let element = element_ref.value();
                        let url = Url::parse(element.attr("href").unwrap()).unwrap();
                        let title = self
                            .info
                            .chapter_title_filter
                            .replace(element.attr("title").unwrap(), "")
                            .trim()
                            .to_string();
                        (url, title)
                    })
                    .collect()
            };

            for element_info in info {
                yield Chapter {
                    url: element_info.0.clone(),
                    title: element_info.1.clone(),
                    language: String::from(""),
                    connector: Connectors::Manganel,
                }
            }
        })
    }

    fn get_pages(&self, chapter: Chapter) -> StreamResult<Page> {
        Box::pin(async_stream::try_stream! {
            let info: Vec<Url> = {
                let dom = {
                    let request = GlobalAPI::global()
                        .client
                        .get(chapter.url.clone())
                        .send()
                        .await
                        .map_err(|err| Error::RequestFail(err.to_string()))?;
                    Html::parse_document(&request.text().await.unwrap())
                };
                dom.select(&self.info.query_pages)
                    .map(|element_ref| {
                        let element = element_ref.value();
                        Url::parse(element.attr("src").unwrap()).unwrap()
                    })
                    .collect()
            };

            for page_url in info {
                yield Page {
                    url: page_url.clone(),
                    referer: chapter.url.clone(),
                    connector: Connectors::Manganel,
                }
            }
        })
    }
}
