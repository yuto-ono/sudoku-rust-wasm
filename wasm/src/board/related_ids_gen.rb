# 各マスの関連セルのインデックスを予め計算してソースコードぬ埋め込むためのスクリプト
BOARD_NUM = 81

File.open("related_ids.rs", "w") do |f|
  f.puts "use super::super::constants::*;"
  f.puts
  f.puts "pub const RELATED_IDS: [[usize; RELATED_LENGTH]; BOARD_NUM] = ["
  BOARD_NUM.times do |pos|
    related_ids = []
    row = pos / 9
    col = pos % 9
    area33top = (row / 3) * 3
    area33left = (col / 3) * 3
    9.times do |i|
      row33 = area33top + (i / 3)
      col33 = area33left + (i % 3)
      related_ids.push(row * 9 + i)
      related_ids.push(i * 9 + col)
      related_ids.push(row33 * 9 + col33)
    end
    related_ids = related_ids.uniq().select{ |id| id != pos }.sort()
    f.puts "    ["
    f.write "        "
    f.write related_ids.join(", ")
    f.puts ","
    f.puts "    ],"
  end
  f.puts "];"
end
