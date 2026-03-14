use pdfium_render::prelude::*;

pub fn render_pdf_page(path: &str) -> image::DynamicImage {
    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./")).unwrap(),
    );

    let doc = pdfium.load_pdf_from_file(path, None).unwrap();
    let page = doc.pages().get(0).unwrap();

    let render = page
        .render_with_config(&PdfRenderConfig::new().set_target_width(800))
        .unwrap();
    
    render.as_image()
}
