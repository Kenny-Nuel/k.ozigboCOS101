use std::io;
use std::io::Write;

struct Leverage {
    shares:f32, liabilities:f32
}

impl Leverage {
    fn lev(&self) -> f32 {
        ((self.shares - self.liabilities) / self.shares) * 100.0
    }
}

fn main() {
    // Database to access a company's history
    println!("----------------------------------------------------------------------");
    println!("HOLIDAY PROJECT");
    println!("----------------------------------------------------------------------");
    println!();

    println!("----------------------------------------------------------------------");
    println!("List of businesses in the database and usernames");
    println!("Cadbury Nigeria PLC - CADB");
    println!("Champion Brewries PLC - CHAM");
    println!("Dangote Sugar Refinery PLC - DANG");
    println!("Flour Mills Nigeria PLC - FLOU");
    println!("Nestle Nigeria PLC - NEST");
    println!("Unilever Nigeria PLC - UNIL");
    println!("Honeywell Nigeria PLC - HONE");
    println!("Nigerian Brewries PLC -  NIGE");
    println!("----------------------------------------------------------------------");
    println!();

    let company_names = vec!["Cadbury Nigeria PLC", "Champion Brewries PLC", "Dangote Sugar Refinery", "Flour Mills Nigeria PLC", "Nestle Nigeria PLC", "Unilever Nigeria PLC", "Honeywell Nigeria PLC", "Nigerian Brewries PLC"];
    let shares = vec![15_000_000.0, 25_000_000.0, 18_000_000.0, 32_000_000.0, 8_000_000.0, 37_000_000.0, 34_000_000.0, 30_000_000.0];
    let liabilities = vec![5_500_000.0, 8_000_000.0, 10_000_000.0, 4_000_000.0, 1_500_000.0, 11_000_000.0, 9_000_000.0, 12_000_000.0];

    let passwords = vec!["cadb", "cham", "dang", "flou", "nest", "unil", "hone", "nige"];

    let mut input1 = String::new();
    println!("Input username: ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let username = input1.trim();

    let mut input2 = String::new();
    println!("Input password: ");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let password = input2.trim();

    if username == "CADB" && password == passwords[0] || username == "CHAM" && password == passwords[1] || username == "DANG" && password == passwords[2] || 
    username == "FLOU" && password == passwords[3] || username == "NEST" && password == passwords[4] || username == "UNIL" && password == passwords[5] || username == "HONE" && password == passwords[6] || username == "NIGE" && password == passwords[7]{
        let mut file = std::fs::File::create("secret.txt").expect("Create failed");
        // QUESTION 1 OF THE CODE

        // CADBURY NIGERIA PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("CADBURY NIGERIA PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1965\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 15,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 5,500,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[0],liabilities:liabilities[0]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // CHAMPION BREWRIES PLC 
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("CHAMPION BREWRIES PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1974\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 15,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 5,500,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[1],liabilities:liabilities[1]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // DANGOTE SUGAR REFINERY PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("DANGOTE SUGAR REFINERY PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1970\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 18,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 10,000,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[2],liabilities:liabilities[2]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // FLOUR MILLS NIGERIA PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("FLOUR MILLS NIGERIA PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1960\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 32,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 4,000,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[3],liabilities:liabilities[3]};
        file.write_all("Percentage Liabilty:  ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // NESTLE NIGERIA PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("NESTLE NIGERIA PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1961\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 8,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 1,500,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[4],liabilities:liabilities[4]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // UNILEVER NIGERIA PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("UNILEVER NIGERIA PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1923\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 37,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 11,000,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[5],liabilities:liabilities[5]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // HONEYWELL NIGERIA PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("HONEYWELL NIGERIA PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1906\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 34,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 9,000,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[6],liabilities:liabilities[6]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // NIGERIAN BREWRIES PLC
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("NIGERIAN BREWRIES PLC\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        file.write_all("Date Founded: 1946\n".as_bytes()).expect("Write failed");
        file.write_all("Total Assets: 30,000,000\n".as_bytes()).expect("Write failed");
        file.write_all("Total Liabilities: 12,000,000\n".as_bytes()).expect("Write failed");
        let percentage = Leverage {shares:shares[7],liabilities:liabilities[7]};
        file.write_all("Percentage Liabilty: ".as_bytes()).expect("Write failed");
        file.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
        file.write_all("%".as_bytes()).expect("Write failed");
        file.write_all("\n".as_bytes()).expect("Write failed");
        file.write_all("----------------------------------------------------------------------".as_bytes()).expect("Write failed");
        file.write_all("\n\n".as_bytes()).expect("Write failed");

        // QUESTION 2 OF THE CODE

        let mut doc = std::fs::File::create("doc.txt").expect("Create failed"); 
        doc.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        doc.write_all("THE PERCENTAGE LEVERAGE OF COMPANIES THAT HAVE SHARES GREATER THAN 20,000,000 \n".as_bytes()).expect("Write failed");
        doc.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");

        let mut count = 0;
        let limit = 7;

        while count != limit {
            count += 1;
            for i in &shares {
                if i > &20_000_000.0 {
                    doc.write_all("Company Name: ".as_bytes()).expect("Write failed");
                    doc.write_all(company_names[count].as_bytes()).expect("Write failed");
                    doc.write_all("\n".as_bytes()).expect("Write failed");
                    let percentage = Leverage {shares:shares[count],liabilities:liabilities[count]};
                    doc.write_all("Percentage Liabilty: \t".as_bytes()).expect("Write failed");
                    doc.write_all(percentage.lev().to_string().as_bytes()).expect("Write failed");
                    doc.write_all("%".as_bytes()).expect("Write failed");
                    doc.write_all("\n".as_bytes()).expect("Write failed");
                    doc.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
                    break;
                }
                else {
                    continue;
                }
            }
        }

        // QUESTION 3 OF THE CODE

        let mut text = std::fs::File::create("text.txt").expect("Create failed"); 
        text.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
        text.write_all("THE PERCENTAGE LEVERAGE OF COMPANIES THAT HAVE SHARES GREATER THAN 20,000,000 \n".as_bytes()).expect("Write failed");
        text.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");

        let mut count = 0;
        let limit = 7;

        while count != limit {
            count += 1;
            for i in &shares {
                if i > &10_000_000.0 {
                    text.write_all("Company Name: ".as_bytes()).expect("Write failed");
                    text.write_all(company_names[count].as_bytes()).expect("Write failed");
                    text.write_all("\n".as_bytes()).expect("Write failed");
                    let percentage = Leverage {shares:shares[count],liabilities:liabilities[count]};
                    let reduced_percentage = (percentage.lev()) * 0.05;
                    text.write_all("Percentage Liabilty: \t".as_bytes()).expect("Write failed");
                    text.write_all(reduced_percentage.to_string().as_bytes()).expect("Write failed");
                    text.write_all("%".as_bytes()).expect("Write failed");
                    text.write_all("\n".as_bytes()).expect("Write failed");
                    text.write_all("----------------------------------------------------------------------\n".as_bytes()).expect("Write failed");
                    break;
                }
                else {
                    continue;
                }
            }
        }

        println!("Access granted");
        println!("Check folder for secret files");

    } else {
        println!("Invalid username or password. Try again.")
    }
}