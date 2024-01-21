use oxide_engine::custom_errors::Errors;

fn main() {
    oxide_engine::logger::init();
    error_test(1);
}

fn error_test(num: i32) -> Result<(), Errors>{
    if num == 1 {
        oxide_engine::logger::info!("Error");
        return Err(Errors::TestError.into());
    }
    Ok(())
}
