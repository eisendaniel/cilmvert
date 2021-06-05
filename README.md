# Batch Image Sequence Convertor in Rust

## Usage

Pass path of directory containing image sequence (of .tiffs for example) and extension to convert to.

`./im_seq_cvtr ~/Pictures/input_img png`

Note: path should exclude trailing `/` and extension exclude leading `.`

## Result

A new directory with converted sequence at `[$origin_dir]_converted_to_[$extension]/`

