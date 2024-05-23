lucky_nums = [4, 8, 15, 16, 23, 42]

# lucky_nums["dog"] # TypeError: no implicit conversion of String into Integer

begin
  # Like try-catch block in Java
  # Run the first line of code, if it fails, run the rescue block
  lucky_nums["dog"]
  num = 10 / 0
rescue ZeroDivisionError
  puts "Division by zero error" # Division by zero error
rescue TypeError => e
  puts e
end
