# CLI Batch Image Convertor in Rust

Uses image + rayon to make a quick and easy batch image sequence reencoder. Every one I found was slow and I didn't need a UI, so this.

## Usage

Pass path of directory containing image sequence (of .tiffs for example) and extension to convert to.

`climvert ~/Pictures/input_img png`

Note: exclude trailing `/` to place in dir next to original, include to place inside original dir.

## Results

Can recode ~1000 tiff to png in ~30sec
