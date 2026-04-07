use crate::{lss_api::ApiResponse};
use crate::structs::SwedenCity;

///this passes if it runs without error.
#[tokio::test]
async fn test_get() {
    
    let _ = ApiResponse::get_from_city(&SwedenCity::Göteborg).await.expect("expected a well formed response");
    
    assert!(true)
}

#[test]
fn test_get_no_tournaments() {
    
    let mock: ApiResponse = ApiResponse::new(0, None, vec![]);
    let temp = mock.get_tournaments();

    assert_eq!(temp.len(), 0)
}