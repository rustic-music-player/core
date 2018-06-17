extern crate rustic_local_provider as provider;

#[test]
fn test_scan() {
    let scanner = provider::Scanner::new("assets");
    let res = scanner.scan().unwrap();

    assert_eq!(res, vec![provider::Track {
        path: "assets/bensound-ukulele.mp3".into(),
        title: "Ukulele".into(),
        artist: Some("Bensound".into()),
        album: None
    }]);
}