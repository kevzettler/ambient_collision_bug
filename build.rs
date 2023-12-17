use ambient_pipeline_types::{
    models::Collider, ModelImporter, ModelsPipeline, Pipeline, PipelineProcessor, PipelinesFile,
};

fn main() {
    PipelinesFile {
        pipelines: vec![
            Pipeline {
                processor: PipelineProcessor::Models(ModelsPipeline {
                    importer: ModelImporter::Regular,
                    output_prefabs: true,
                    collider: Collider::FromModel {
                        flip_normals: false,
                        reverse_indices: false,
                    },
                    collider_type: ambient_pipeline_types::models::ColliderType::Dynamic,
                    ..Default::default()
                }),
                sources: vec!["cube.glb".to_string()],
                tags: vec![],
                categories: vec![],
            },
            Pipeline {
                processor: PipelineProcessor::Models(ModelsPipeline {
                    importer: ModelImporter::Regular,
                    output_prefabs: true,
                    collider: Collider::FromModel {
                        flip_normals: false,
                        reverse_indices: false,
                    },
                    collider_type: ambient_pipeline_types::models::ColliderType::Static,
                    ..Default::default()
                }),
                sources: vec!["ground.glb".to_string()],
                tags: vec![],
                categories: vec![],
            },
        ],
    }
    .save_to_file("assets/pipeline.toml")
    .unwrap();
}
