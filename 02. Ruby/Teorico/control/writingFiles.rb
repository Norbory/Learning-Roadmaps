File.open("core/employees.txt", "a+") do |file|
  if file.read().include? "Oscar"
    puts "Oscar is already in the file"
  else
    file.write("\nOscar, Accounting")
  end
end

File.open("core/employees.txt", "r+") do |file|
  file.readline()
  file.write("Sebatian, CEO")
end

File.open("core/index.html", "w") do |file|
  file.write("<h1>Office Schema</h1>")
end

# w - write (will overwrite the entire file)
# a - append (will add on to whatever is in the file)
# r - reading
# w+ - writing and reading
# a+ - open a file for reading and appending
# r+ - reading and writing
# b - binary mode
# t - text mode (default)
