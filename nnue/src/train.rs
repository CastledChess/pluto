use bullet::{
    nn::{optimiser, Activation},
    trainer::{
        default::{
            formats::sfbinpack::{
                chess::{piecetype::PieceType, r#move::MoveType},
                TrainingDataEntry,
            },
            inputs, loader, outputs, Loss, TrainerBuilder,
        },
        schedule::{lr, wdl, TrainingSchedule, TrainingSteps},
        settings::LocalSettings,
    },
};

pub const FEATURES: usize = 768;
pub const HIDDEN: usize = 1024;

// Clipped ReLu bounds
pub const CR_MIN: i16 = 0;
pub const CR_MAX: i16 = 255;

// Quantization factors
pub const QA: i16 = 255;
pub const QB: i16 = 64;

// Eval scaling factor
pub const SCALE: i32 = 400;

pub fn train() {
    let mut trainer = TrainerBuilder::default()
        .quantisations(&[QA, QB])
        .optimiser(optimiser::AdamW)
        .loss_fn(Loss::SigmoidMSE)
        .input(inputs::Chess768)
        .output_buckets(outputs::Single)
        .feature_transformer(HIDDEN)
        .activate(Activation::SCReLU)
        .add_layer(1)
        .build();

    let schedule = TrainingSchedule {
        net_id: "(768-1024)x2-1_screlu".to_string(),
        eval_scale: SCALE as f32,
        steps: TrainingSteps {
            batch_size: 16_384,
            batches_per_superbatch: 6104,
            start_superbatch: 1,
            end_superbatch: 250,
        },
        wdl_scheduler: wdl::ConstantWDL { value: 0.6 },
        lr_scheduler: lr::StepLR {
            start: 0.002,
            gamma: 0.1,
            step: 10,
        },
        save_rate: 5,
    };

    trainer.set_optimiser_params(optimiser::AdamWParams::default());

    let settings = LocalSettings {
        threads: 32,
        test_set: None,
        output_directory: "checkpoints",
        batch_queue_size: 64,
    };

    let data_loader = {
        let file_path = "C:/Users/ludov/Documents/test80-2024-02-feb-2tb7p.min-v2.v6.binpack";
        let buffer_size_mb = 1024;
        let threads = 4;
        fn filter(entry: &TrainingDataEntry) -> bool {
            entry.ply >= 16
                && !entry.pos.is_checked(entry.pos.side_to_move())
                && entry.score.unsigned_abs() <= 10000
                && entry.mv.mtype() == MoveType::Normal
                && entry.pos.piece_at(entry.mv.to()).piece_type() == PieceType::None
        }

        loader::SfBinpackLoader::new(file_path, buffer_size_mb, threads, filter)
    };

    trainer.run(&schedule, &settings, &data_loader);
}
