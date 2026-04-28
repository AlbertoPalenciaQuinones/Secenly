use cryptographic_message_syntax::SignedDataBuilder;

pub struct SignedDataService;

impl SignedDataService {

    pub fn signed_data(license_der_bytes: Vec<u8>) {
        let cms_der = SignedDataBuilder::default()
                        .content_inline(license_der_bytes)
                        .build_der()
                        .expect("Error construyendo CMS SignedData");
        println!("CMS SignedData DER: {:02x?}", cms_der);
    }
}

