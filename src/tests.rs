use crate::{
    endpoint::{FitIn, ResponseMode},
    EndpointBuilder, Filter, Server,
};

const TEST_BASE: &str = "http://my.server.com";
const SECURITY_KEY: &str = "my-security-key";
const IMAGE_PATH: &str = "my.server.com/some/path/to/image.jpg";

fn new_builder() -> EndpointBuilder {
    Server::new(TEST_BASE, SECURITY_KEY)
        .expect("Server creation failed")
        .endpoint_builder()
}

#[test]
fn signing_of_a_known_url_results() {
    let width = 300;
    let height = 200;

    let endpoint = new_builder().resize((width, height)).build();

    let path = endpoint.to_path(IMAGE_PATH);

    assert_eq!(
        path,
        "/8ammJH8D-7tXy6kU3lTvoXlhu4o=/300x200/my.server.com/some/path/to/image.jpg"
    );
}

#[test]
fn signature_with_meta() {
    let endpoint = new_builder().response(ResponseMode::Metadata).build();

    let path = endpoint.to_path(IMAGE_PATH);

    assert_eq!(
        path,
        "/Ps3ORJDqxlSQ8y00T29GdNAh2CY=/meta/my.server.com/some/path/to/image.jpg"
    );
}

#[test]
fn signature_with_smart() {
    let endpoint = new_builder().smart(true).build();

    let path = endpoint.to_path(IMAGE_PATH);

    assert_eq!(
        path,
        "/-2NHpejRK2CyPAm61FigfQgJBxw=/smart/my.server.com/some/path/to/image.jpg"
    );
}

#[test]
fn signature_with_fit_in() {
    let endpoint = new_builder().fit_in(FitIn::Default).build();

    let path = endpoint.to_path(IMAGE_PATH);

    assert_eq!(
        path,
        "/uvLnA6TJlF-Cc-L8z9pEtfasO3s=/fit-in/my.server.com/some/path/to/image.jpg"
    );
}

#[test]
fn signature_with_filters() {
    let endpoint = new_builder()
        .filters([Filter::Brightness(10), Filter::Contrast(20)])
        .build();

    let path = endpoint.to_path(IMAGE_PATH);

    assert_eq!(
        path,
        "/ZZtPCw-BLYN1g42Kh8xTcRs0Qls=/filters:brightness(10):contrast(20)/my.server.com/some/path/to/image.jpg"
    );
}
