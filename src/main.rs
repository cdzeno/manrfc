use reqwest;
use minus::Pager;
use clap::{App, Arg};
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

#[macro_use] extern crate prettytable;
use prettytable::Table;

const RFC_EDITOR: &str = "https://www.rfc-editor.org/search/rfc_search_detail.php?";
const RFC_TXT: &str = "http://www.rfc-editor.org/rfc/";

fn main() {
    let matches = App::new("manrfc")
        .version("0.1.0")    
        .author("Gabriele Quagliarella (@cdzeno)")
        .about("Manual pager for RFC-Editor")
        .arg(
            Arg::with_name("number")
            .short("n")
            .long("number")
            .value_name("NUMBER")
            .help("RFC number to view")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("title")
            .short("t")
            .long("title")
            .value_name("TITLE")
            .help("RFC title to search")
            .takes_value(true)
        )
        .arg(
            Arg::with_name("results")
            .short("r")
            .long("results")
            .value_name("RESULTS")
            .help("Number of results to show")
            .default_value("10")
            .takes_value(true)
        )
        .get_matches();
    
    let mut search_url: String = String::from(RFC_EDITOR);
    let mut result_table = Table::new();

    // Parse arguments:
    if let Some(v) = matches.value_of("number"){
        // If the RFC number is specified then search for it!
        let mut rfc_txt_url: String = String::from(RFC_TXT);
        let mut rfc_is_valid: bool = false;

        search_url.push_str(&format!("rfc={}&", v));

        // Make HTTP GET request to RFC-Editor and searching for single RFC
        let rfc_res = reqwest::blocking::get(&search_url).unwrap();
        assert!(rfc_res.status().is_success());

        // Initialize a new Document class
        let rfc_html = Document::from_read(rfc_res).unwrap();

        for table in rfc_html.find(Class("gridtable").descendant(Name("tbody"))) {
            for row in table.find(Name("tr")).skip(1) {
                
                let number = row.find(Name("a"))
                                .next()
                                .unwrap()
                                .text();

                // If the "Number" field returned into the HTML page is equal 
                // to the RFC number inserted cmd parameter then the search
                // had a good response, otherwise is not valid 
                match number.find(v) {
                    Some(_) => { 
                        rfc_txt_url.push_str(&format!("rfc{}.txt", v));
                        rfc_is_valid = true;
                    }
                    _ => () 
                }
            }
        }

        if rfc_is_valid {
            // If the RFC is valid then retrieve the txt file from RFC-Editor
            let res_rfc_text = reqwest::blocking::get(&rfc_txt_url).unwrap().text().unwrap();

            // Initialize a static Pager for printing RFC text
            let rfc_pager = Pager::new().set_prompt(format!("RFC {}", v));
            
            // Set the Pager's text with the RFC text
            let rfc_paged = rfc_pager.set_text(res_rfc_text);

            // Show the RFC text with the Pager
            minus::page_all(rfc_paged).unwrap();

            std::process::exit(0);
        } else {
            println!("RFC not found!");
            std::process::exit(-1);
        }
    }
    
    if let Some(v) = matches.value_of("results") {
        search_url.push_str(&format!("page={}&", v));
    }

    if let Some(v) = matches.value_of("title") {
        search_url.push_str(&format!("title={}&", v));
    } 
     
    // Make HTTP GET request to RFC-Editor and check the response status
    let res = reqwest::blocking::get(search_url).unwrap();
    assert!(res.status().is_success());

    // Initialize a new Document class
    let html = Document::from_read(res).unwrap();

    for table in html.find(Class("gridtable").descendant(Name("tbody"))) {
        // Insert the first static line of the table:
        result_table.add_row(row!["Number", "Title", "Authors", "Date"]);

        // Parse all the table's row skipping the first one
        for row in table.find(Name("tr")).skip(1) {
            // RFC `Number`
            let number = row.find(Name("a"))
                            .next()
                            .unwrap()
                            .text();

            // RFC `Title`
            let title = row.find(Name("td"))
                           .nth(2)
                           .unwrap()
                           .text();

            // RFC `Authors`
            let authors = row.find(Name("td"))
                             .nth(3)
                             .unwrap()
                             .text();
            
            // RFC `Date`
            let date = row.find(Name("td"))
                          .nth(4)
                          .unwrap()
                          .text();
          
            // Add RFC line to the result table
            result_table.add_row(row![number, title, authors, date]);
        }
    }

    result_table.printstd();
}
