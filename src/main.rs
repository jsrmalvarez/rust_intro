

mod library { // <- barely resemples a namespace

	#[derive(Debug)] // <- Automagically implements the fmt::Debug trait to format the data for debugging purposes
	enum Format {
		Epub(u32), // <- those are variants
		Pdf(u32),
		Paper(u64)
	}

	#[derive(Debug)]
	pub enum Book {
		Ebook{
			title: String,
			author: String,
			format: Format,
		},
		Physical{
			title: String,
			author: String,
			format: Format,
			weight_g: u16,
		}
	}


  pub struct Library{
    byId:
  }

	// impl blocks allow us to add behaviour to enums and structs
	impl Book {

		pub fn new_physical_book(title: String, author:String, n_pages:u64, weight_g: u16) -> Self {

			// Doesn't it look like javascript?
			Book::Physical{
				author,
				title,
				format: Format::Paper(n_pages),
				weight_g,
			}
		}

		pub fn new_pdf_ebook(title: &str, author:&str, size_ki_b: u32) -> Self {
			Self::new_ebook(title, author, Format::Pdf(size_ki_b))
		}

		pub fn new_epub_ebook(title: &str, author:&str,size_ki_b: u32) -> Self {
			Self::new_ebook(title, author, Format::Epub(size_ki_b))
		}

		fn new_ebook(title: &str, author:&str, format:Format) -> Self {
			match format {
				Format::Epub(_) | Format::Pdf(_) => Book::Ebook{title:title.to_owned(), author:author.to_owned(), format},
				_ => panic!("Invalid format"),
			}
		}

	}
}


use crate::library::Book;

fn main() {


	let title = "Nacidos para aprender";
	let author = "Fernando Alonso Martín";
	let book = Book::new_epub_ebook(title,
						   										author,
																	40*1_024);

	// deriving Debug trait enables us to print a Book
	// as you would print a javascript object using console.log()
	println!("{:?}", book);
}


#[cfg(test)]
mod test{

	use super::*;

	#[test]
	fn test_a (){
		assert!(true);
	}

}
