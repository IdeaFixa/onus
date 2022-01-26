use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "metadata")]
#[serde(rename_all = "camelCase")]
pub struct MavenMetadata {
    group_id: String,
    artifact_id: String,
    versioning: Versioning,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Versioning {
    pub latest: String,
    pub release: String,
    pub versions: Versions,
    pub last_updated: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Versions {
    pub version: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename = "project")]
#[serde(rename_all = "camelCase")]
pub struct MavenPom {
    pub dependencies: Dependencies,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Dependencies {
    pub dependency: Vec<Dependency>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub scope: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::maven::Dependency;
    use crate::maven::MavenMetadata;
    use crate::maven::MavenPom;
    use crate::maven::Versions;
    use serde_xml_rs::from_str;

    #[test]
    fn test_maven_metadata_parse() {
        let data = include_str!("../test/maven/maven-metadata.xml");

        let metadata: MavenMetadata = from_str(data).expect("Failed to parse metadata file");
        let versioning = metadata.versioning;

        assert_eq!(metadata.group_id, "dev.zio");
        assert_eq!(metadata.artifact_id, "zio_3");
        assert_eq!(versioning.latest, "2.0.0-RC1");
        assert_eq!(versioning.release, "2.0.0-RC1");
        assert_eq!(
            versioning.versions,
            Versions {
                version: vec![
                    "1.0.8".to_string(),
                    "1.0.9".to_string(),
                    "1.0.10".to_string(),
                    "1.0.11".to_string(),
                    "2.0.0-RC1".to_string()
                ],
            }
        );
        assert_eq!(versioning.last_updated, "20211214233703");
    }

    #[test]
    fn test_maven_pom_parse() {
        let data = include_str!("../test/maven/pom.xml");

        let pom: MavenPom = from_str(data).expect("Failed to parse pom file");
        let dependencies = pom.dependencies;

        assert_eq!(
            dependencies.dependency,
            vec![
                Dependency {
                    group_id: "dev.zio".to_string(),
                    artifact_id: "zio-stacktracer_3".to_string(),
                    version: "1.0.13".to_string(),
                    scope: None
                },
                Dependency {
                    group_id: "org.scala-lang".to_string(),
                    artifact_id: "scala3-library_3".to_string(),
                    version: "3.1.0".to_string(),
                    scope: None
                },
                Dependency {
                    group_id: "org.openjdk.jcstress".to_string(),
                    artifact_id: "jcstress-core".to_string(),
                    version: "0.3".to_string(),
                    scope: Some("test".to_string())
                },
                Dependency {
                    group_id: "com.github.ghik".to_string(),
                    artifact_id: "silencer-lib_2.13.7".to_string(),
                    version: "1.7.7".to_string(),
                    scope: Some("provided".to_string())
                },
            ]
        )
    }
}
