use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub fn new_pb(mpb: &mut MultiProgress, prefix: String, len: u64) -> ProgressBar {
    // Create a new progress bar style.
    let style = "[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}";
    let pb_style = ProgressStyle::with_template(style)
        .unwrap()
        .progress_chars("##-");

    // Add the new progress bar into the multi progress bar.
    let pb = mpb.add(ProgressBar::new(len));
    pb.set_style(pb_style);
    pb
}
