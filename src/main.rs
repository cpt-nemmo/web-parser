use reqwest;

fn main() {
    // download the target HTML document
    let response = reqwest::blocking::get("https://scrapeme.live/shop/").unwrap();
    // get the HTML content from the request response
    // and print it
    let html_content = response.text().unwrap();

    // println!("{html_content}");

    let document = scraper::Html::parse_document(&html_content);
    let html_product_selector = scraper::Selector::parse("li.product").unwrap();
    let html_products = document.select(&html_product_selector);


    #[derive(Debug)]
    struct PokemonProduct {
        url: Option<String>,
        image: Option<String>,
        name: Option<String>,
        price: Option<String>,
    }


    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();
 
    for html_product in html_products {
        // scraping logic to retrieve the info
        // of interest
        let url = html_product
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);
        let image = html_product
            .select(&scraper::Selector::parse("img").unwrap())
            .next()
            .and_then(|img| img.value().attr("src"))
            .map(str::to_owned);
        let name = html_product
            .select(&scraper::Selector::parse("h2").unwrap())
            .next()
            .map(|h2| h2.text().collect::<String>());
        let price = html_product
            .select(&scraper::Selector::parse(".price").unwrap())
            .next()
            .map(|price| price.text().collect::<String>());
    
        // instantiate a new Pokemon product
        // with the scraped data and add it to the list
        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };
        pokemon_products.push(pokemon_product);
        // println!("{:?}", pokemon_products)
    }



}