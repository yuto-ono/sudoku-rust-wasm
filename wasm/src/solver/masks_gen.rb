# 各マスの関連セルのインデックスを予め計算してソースコードぬ埋め込むためのスクリプト
File.open("masks.rs", "w") do |f|
  f.puts "// このファイルは masks_gen.rb により自動生成されました"
  f.puts
  f.puts "use super::super::constants::*;"
  f.puts
  f.puts "pub const MASKS: [u128; BOARD_NUM] = ["
  81.times do |pos|
    mask = 0
    row = pos / 9
    col = pos % 9
    area33top = (row / 3) * 3
    area33left = (col / 3) * 3
    9.times do |i|
      row33 = area33top + (i / 3)
      col33 = area33left + (i % 3)
      mask |= 1 << (row * 9 + i)
      mask |= 1 << (i * 9 + col)
      mask |= 1 << (row33 * 9 + col33)
    end
    f.puts "    #{mask},"
  end
  f.puts "];"
end
