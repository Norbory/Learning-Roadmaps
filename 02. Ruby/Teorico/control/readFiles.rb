File.open("core/employees.txt", "r") do |file|

  # Once you read the file in a method you can't read it again in another method
  # puts file.read().include? "Angelo" # true
  # puts file.read().include? "Jim" # false
  # puts file.readline() # Angelo, Sales
  # puts file.readchar() # A

  for line in file.readlines()
    puts line
  end
end

# Or

file = File.open("core/employees.txt", "r")
puts file.read
# You have to close the file after you open it
file.close()
