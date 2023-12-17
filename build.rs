use ambient_pipeline_types::{
    models::{Collider, ColliderType},
    MaterialsImporter, MaterialsPipeline, ModelImporter, ModelsPipeline, Pipeline,
    PipelinePbrMaterial, PipelineProcessor, PipelinesFile,
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
