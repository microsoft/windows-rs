use windows_registry::*;
use windows_result::*;
use windows_sys::Win32::Foundation::*;

#[test]
fn create_with_transaction() {
    let test_key = "software\\windows-rs\\tests\\transaction";
    _ = CURRENT_USER.remove_tree(test_key);
    let key = CURRENT_USER.create(test_key).unwrap();

    let tx = Transaction::new().unwrap();

    let tx_key = CURRENT_USER
        .options()
        .transaction(&tx)
        .read()
        .write()
        .open(test_key)
        .unwrap();

    tx_key.set_u64("u64", 123u64).unwrap();
    assert_eq!(tx_key.get_u64("u64").unwrap(), 123u64);

    // The transaction is not yet committed so this non-transaction read will fail.
    assert_eq!(
        key.get_u64("u64").unwrap_err().code(),
        HRESULT::from_win32(ERROR_FILE_NOT_FOUND)
    );

    tx.commit().unwrap();

    // Now that the transaction is committed the non-transaction read will succeed.
    assert_eq!(key.get_u64("u64").unwrap(), 123u64);

    // The transaction is no longer active so this key cannot be used.
    assert_eq!(
        tx_key.get_u64("u64").unwrap_err().code(),
        HRESULT::from_win32(ERROR_TRANSACTION_NOT_ACTIVE)
    );
}
