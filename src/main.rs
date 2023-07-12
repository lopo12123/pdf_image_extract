use std::fs::{create_dir, File};
use std::path::Path;
use image::ImageFormat;
use pdfium_render::prelude::*;

// let dll_path = r"D:\pdf_parser\src\pdfium.dll";

// let target_pdf = "test/acsami.7b11092.pdf";
// let outdir_name = "./acsami";

// let target_pdf = "test/872421397224226817.pdf";
// let outdir_name = "./872421397224226817";

// let target_pdf = "test/One-step PDA coating strategy on pure Zn for blood-contacting engineering.pdf";
// let outdir_name = "./PDA";

// let target_pdf = "test/image-test.pdf";
// let outdir_name = "./image-test";

pub fn main() -> Result<(), PdfiumError> {
    let dll_path = r"D:\GitProject\pdf_image_extract\src\pdfium.dll";

    let target_pdf = "test/One-step PDA coating strategy on pure Zn for blood-contacting engineering.pdf";
    let outdir_name = "./PDA";

    let out_dir = Path::new(outdir_name);
    if !out_dir.exists() {
        match create_dir(out_dir) {
            Ok(_) => println!("The output folder does not exist, it was created automatically."),
            Err(err) => println!("The output folder does not exist, automatic creation failed.\n{}", err)
        };
    }

    match Pdfium::bind_to_library(dll_path) {
        Ok(binding) => {
            let instance = Pdfium::new(binding);
            let pdf_doc = instance.load_pdf_from_file(target_pdf, None)?;

            println!("\n=============== Metadata ===============");

            pdf_doc.metadata().iter().for_each(|item| {
                println!("{:?}", item);
            });

            pdf_doc.pages()
                .iter()
                .enumerate()
                .for_each(|(page_index, page)| {
                    // For each page in the document, output the images on the page to separate files.

                    println!("\n=============== Page {page_index} ===============");

                    page.objects()
                        .iter()
                        .enumerate()
                        .for_each(|(object_index, object)| {
                            if let Some(image) = object.as_image_object() {
                                if let Ok(image) = image.get_processed_image(&pdf_doc) {
                                    println!("Exporting image with object index {object_index} to file");

                                    match image
                                        .save_with_format(
                                            format!("{outdir_name}/ImageExtra_page_{page_index}_image_{object_index}.jpg"),
                                            ImageFormat::Jpeg,
                                        ) {
                                        Ok(_) => println!("OK."),
                                        Err(err) => println!("Err. {}", err)
                                    }
                                }

                                let next_line = (&page).objects().get(object_index + 1)
                                    .map_or("".to_string(), |v| v.as_text_object().map_or("".to_string(), |t| t.text()));
                                println!("next_line: {}", next_line);

                                // if let Some(image_desc) = next_line {
                                //     println!("{}", image_desc.text());
                                // }
                            }
                        });
                });
        }
        Err(err) => {
            println!("failed to binding!\n{}", err);
        }
    }

    Ok(())
}