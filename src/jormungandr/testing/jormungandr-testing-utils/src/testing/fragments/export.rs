use super::FragmentNode;
use crate::wallet::Wallet;
use chain_core::property::Deserialize;
use chain_impl_mockchain::fragment::{Fragment, FragmentId};
use chrono::{DateTime, Utc};
use jormungandr_lib::interfaces::Address;
use std::io::Write;
use std::{fs, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FragmentExporterError {
    #[error("cannot create dump folder {0}")]
    CannotCreateDumpFolder(PathBuf),
    #[error("cannot create dump file {0}")]
    CannotCreateDumpFile(PathBuf),
    #[error("cannot write fragment bin to file {0}")]
    CannotWriteFragmentToDumpFile(PathBuf),
    #[error("io error")]
    IoError(#[from] std::io::Error),
}

pub struct FragmentExporter {
    dump_folder: PathBuf,
}

impl FragmentExporter {
    pub fn new(dump_folder: PathBuf) -> Result<Self, FragmentExporterError> {
        fs::create_dir_all(dump_folder.clone())
            .map_err(|_| FragmentExporterError::CannotCreateDumpFolder(dump_folder.clone()))?;

        Ok(Self { dump_folder })
    }

    pub fn read(&self) -> Result<Vec<Fragment>, FragmentExporterError> {
        self.read_as_bytes()?
            .iter()
            .map(|bytes| Ok(Fragment::deserialize(bytes.as_ref()).unwrap()))
            .collect()
    }

    pub fn read_as_bytes(&self) -> Result<Vec<Vec<u8>>, FragmentExporterError> {
        fs::read_dir(&self.dump_folder)?
            .into_iter()
            .filter(|r| {
                let path = r.as_ref().unwrap().path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                file_name.contains("_from_")
                    && file_name.contains("_to_")
                    && file_name.ends_with(".txt")
            })
            .map(|r| {
                let path = r.as_ref().unwrap().path();
                let content = jortestkit::prelude::read_file(path);
                let bytes = hex::decode(content.trim()).unwrap();
                Ok(bytes)
            })
            .collect()
    }

    pub fn dump_to_file(
        &self,
        fragment: &Fragment,
        sender: &Wallet,
        via: &dyn FragmentNode,
    ) -> Result<(), FragmentExporterError> {
        let file_name = self.generate_file_name(fragment, sender, via);
        let file_path = self.dump_folder.join(file_name);
        let mut file = fs::File::create(&file_path)
            .map_err(|_| FragmentExporterError::CannotCreateDumpFile(file_path))?;

        file.write_all(&self.format_fragment(fragment).as_bytes())
            .map_err(|_| {
                FragmentExporterError::CannotWriteFragmentToDumpFile(self.dump_folder.clone())
            })?;

        Ok(())
    }

    fn generate_file_name(
        &self,
        fragment: &Fragment,
        sender: &Wallet,
        via: &dyn FragmentNode,
    ) -> String {
        let now: DateTime<Utc> = Utc::now();
        let alias = {
            if via.alias().is_empty() {
                "jormungandr"
            } else {
                via.alias()
            }
        };

        format!(
            "{}_{}_from_{}_to_{}.txt",
            now.format("%F_%H_%M_%S_%f"),
            self.format_id(fragment.hash()),
            self.format_address(sender.address()),
            alias
        )
    }

    fn format_fragment(&self, fragment: &Fragment) -> String {
        use chain_core::property::Serialize;

        let bytes = fragment.serialize_as_vec().unwrap();
        hex::encode(&bytes)
    }

    fn format_address(&self, address: Address) -> String {
        self.format_hash(address.to_string())
    }

    fn format_id(&self, id: FragmentId) -> String {
        self.format_hash(id.to_string())
    }

    fn format_hash(&self, hash: String) -> String {
        hash
    }
}
