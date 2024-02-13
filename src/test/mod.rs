 mod user;
 mod beneficiary;
 mod details;
 mod stats;
mod category;

 #[cfg(test)]
#[tokio::test]
async fn test(){
    user::create_user().await;
    user::update_user().await;
    beneficiary::create_beneficiary().await;
    beneficiary::update_beneficiary().await;
    beneficiary::select_beneficiary().await;
    beneficiary::select_beneficiaries().await;
    details::insert_allergy().await;
    details::insert_presence().await;
    details::insert_note().await;
    details::update_note().await;
    details::select_details().await;
    user::delete_user().await;
}