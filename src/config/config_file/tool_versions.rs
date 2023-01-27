use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use indexmap::IndexMap;

use crate::config::config_file::{ConfigFile, ConfigFileType};
use crate::plugins::{PluginName, PluginSource};

// python 3.11.0 3.10.0
// shellcheck 0.9.0
// shfmt 3.6.0

/// represents asdf's .tool-versions file
#[derive(Debug, Default)]
pub struct ToolVersions {
    path: PathBuf,
    pre: String,
    plugins: IndexMap<PluginName, ToolVersionPlugin>,
}

#[derive(Debug, Default)]
struct ToolVersionPlugin {
    versions: Vec<String>,
    post: String,
}

impl ToolVersions {
    pub fn init(filename: &Path) -> ToolVersions {
        ToolVersions {
            path: filename.to_path_buf(),
            ..Default::default()
        }
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        trace!("parsing tool-versions: {}", path.display());
        Ok(Self {
            path: path.to_path_buf(),
            ..Self::parse_str(&read_to_string(path)?)?
        })
    }

    pub fn parse_str(s: &str) -> Result<Self> {
        let mut pre = String::new();
        for line in s.lines() {
            if !line.trim_start().starts_with('#') {
                break;
            }
            pre.push_str(line);
            pre.push('\n');
        }

        Ok(Self {
            path: PathBuf::new(),
            plugins: Self::parse_plugins(s)?,
            pre,
        })
    }

    fn get_or_create_plugin(&mut self, plugin: &str) -> &mut ToolVersionPlugin {
        self.plugins.entry(plugin.to_string()).or_default()
    }

    fn parse_plugins(input: &str) -> Result<IndexMap<PluginName, ToolVersionPlugin>> {
        let mut plugins: IndexMap<PluginName, ToolVersionPlugin> = IndexMap::new();
        for line in input.lines() {
            if line.trim_start().starts_with('#') {
                if let Some(prev) = &mut plugins.values_mut().last() {
                    prev.post.push_str(line);
                    prev.post.push('\n');
                }
                continue;
            }
            let (line, post) = line.split_once('#').unwrap_or((line, ""));
            let mut parts = line.split_whitespace();
            if let Some(plugin) = parts.next() {
                // handle invalid trailing colons in `.tool-versions` files
                // note that this method will cause the colons to be removed
                // permanently if saving the file again, but I think that's fine
                let plugin = plugin.trim_end_matches(':');

                let tvp = ToolVersionPlugin {
                    versions: parts.map(|v| v.to_string()).collect(),
                    post: match post {
                        "" => String::from("\n"),
                        _ => [" #", post, "\n"].join(""),
                    },
                };
                plugins.insert(plugin.to_string(), tvp);
            }
        }
        Ok(plugins)
    }
}

impl Display for ToolVersions {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.dump())
    }
}

impl ConfigFile for ToolVersions {
    fn get_type(&self) -> ConfigFileType {
        ConfigFileType::ToolVersions
    }

    fn get_path(&self) -> &Path {
        self.path.as_path()
    }

    fn source(&self) -> PluginSource {
        PluginSource::ToolVersions(self.path.clone())
    }

    fn plugins(&self) -> IndexMap<PluginName, Vec<String>> {
        self.plugins
            .iter()
            .map(|(plugin, tvp)| (plugin.clone(), tvp.versions.clone()))
            .collect()
    }

    fn env(&self) -> HashMap<PluginName, String> {
        HashMap::new()
    }

    fn remove_plugin(&mut self, plugin: &PluginName) {
        self.plugins.remove(plugin);
    }

    fn add_version(&mut self, plugin: &PluginName, version: &str) {
        self.get_or_create_plugin(plugin)
            .versions
            .push(version.to_string());
    }

    fn replace_versions(&mut self, plugin_name: &PluginName, versions: &[String]) {
        self.get_or_create_plugin(plugin_name).versions.clear();
        for version in versions {
            self.add_version(plugin_name, version);
        }
    }

    fn save(&self) -> Result<()> {
        let s = self.dump();
        Ok(fs::write(&self.path, s)?)
    }

    fn dump(&self) -> String {
        let mut s = self.pre.clone();

        for (plugin, tv) in &self.plugins {
            s.push_str(&format!("{} {}{}", plugin, tv.versions.join(" "), tv.post));
        }

        s.trim_end().to_string() + "\n"
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::fmt::Debug;

    use indoc::indoc;
    use insta::{assert_display_snapshot, assert_snapshot};
    use pretty_assertions::assert_eq;

    use crate::dirs;

    use super::*;

    #[test]
    fn test_parse() {
        let tv = ToolVersions::from_file(dirs::CURRENT.join(".tool-versions").as_path()).unwrap();
        assert_eq!(tv.path, dirs::CURRENT.join(".tool-versions"));
        assert_display_snapshot!(tv, @r###"
        #python 3.11.1 3.10.9 # foo
        shellcheck 0.9.0
        shfmt 3.6.0 # test comment
        #nodejs 18.13.0
        nodejs system
        "###);
    }

    #[test]
    fn test_parse_comments() {
        let orig = indoc! {"
        # intro comment
        python 3.11.0 3.10.0 # some comment # more comment
        #shellcheck 0.9.0
        shfmt 3.6.0
        # tail comment
        "};
        let tv = ToolVersions::parse_str(orig).unwrap();
        assert_eq!(tv.dump(), orig);
    }

    #[test]
    fn test_parse_colon() {
        let orig = indoc! {"
        ruby: 3.0.5
        "};
        let tv = ToolVersions::parse_str(orig).unwrap();
        assert_snapshot!(tv.dump(), @r###"
        ruby 3.0.5
        "###);
    }

    #[derive(Debug)]
    pub struct MockToolVersions {
        pub path: PathBuf,
        pub plugins: IndexMap<PluginName, Vec<String>>,
        pub env: HashMap<String, String>,
    }

    // impl MockToolVersions {
    //     pub fn new() -> Self {
    //         Self {
    //             path: PathBuf::from(".tool-versions"),
    //             plugins: IndexMap::from([
    //                 (
    //                     "python".to_string(),
    //                     vec!["3.11.0".to_string(), "3.10.0".to_string()],
    //                 ),
    //                 ("shellcheck".to_string(), vec!["0.9.0".to_string()]),
    //                 ("shfmt".to_string(), vec!["3.6.0".to_string()]),
    //             ]),
    //             env: HashMap::from([
    //                 ("FOO".to_string(), "bar".to_string()),
    //                 ("BAZ".to_string(), "qux".to_string()),
    //             ]),
    //         }
    //     }
    // }
    //
    impl Display for MockToolVersions {
        fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    impl ConfigFile for MockToolVersions {
        fn get_type(&self) -> ConfigFileType {
            ConfigFileType::ToolVersions
        }

        fn get_path(&self) -> &Path {
            self.path.as_path()
        }

        fn source(&self) -> PluginSource {
            todo!()
        }

        fn plugins(&self) -> IndexMap<PluginName, Vec<String>> {
            self.plugins.clone()
        }

        fn env(&self) -> HashMap<String, String> {
            self.env.clone()
        }

        fn remove_plugin(&mut self, _plugin_name: &PluginName) {
            todo!()
        }

        fn add_version(&mut self, _plugin_name: &PluginName, _version: &str) {
            todo!()
        }

        fn replace_versions(&mut self, _plugin_name: &PluginName, _versions: &[String]) {
            todo!()
        }

        fn save(&self) -> Result<()> {
            todo!()
        }

        fn dump(&self) -> String {
            todo!()
        }
    }
}