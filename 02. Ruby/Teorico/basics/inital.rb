# This is the first file that will be executed when the program is run.
# puts "Hello World!"

# Simple program
# print "/___|"

# Variables
# A variable is a container that stores a value.
character_name = "John"
character_age = '35'
# print 'There were once a ' + character_age + ' man named ' + character_name

# Data Types
# ----------------
# Strings
name = 'Mike'
# Integers
age = 24
# Floats
gpa = 3.4121
# Boolean
admitted = true
# Nil
flaws = nil
# ----------------

# Strings methods
# ----------------
frase = 'Hello World'
# Length
puts frase.length
# Upcase
puts frase.upcase
# Downcase
puts frase.downcase
# Strip
frase = '     Hello World     '
puts frase.strip
# Split
puts frase.split(' ')
# Index
frase = 'Hello World'
puts frase.index('W')
# Sub
puts frase.sub('World', 'Universe')
# Get character
puts frase[0]
# Get substring
puts frase[0, 5]
# Include
puts frase.include? 'World'
# ----------------

# Math and Numbers
# ----------------
puts 5 + 9
puts 1.0 + 7
puts 2**3
puts 10 / 7
puts 10 / 7.0
puts 10 * 7
puts 10 * 7.0
puts 10 % 3
num = -20.5
puts num.abs()
puts num.round()
puts num.ceil()
puts num.floor()
puts num.to_s()
puts Math.sqrt(36)
puts Math.log(1)
# ----------------

# User Input
# ----------------
# puts 'Enter your name: '
# name = gets.chomp
# puts 'Enter your age: '
# age = gets
# puts 'Hello ' + name + ', you are ' + age
# chomp removes the new line character
# ----------------

# Arrays
# ----------------
friends = Array['Kevin', 'Karen', 'Oscar']
# Can store different data types
print friends
puts friends[0]
puts friends[-3]
puts friends[0, 2]
friends[0] = 'Dwight'
puts friends
# Initialize an empty array
# amigos = Array.new
# amigos[0] = 'Michael'
# amigos[5] = 'Jim'
# puts amigos

# Methods
puts friends.include? 'Karen'
puts friends.reverse()
puts friends.sort()
puts friends.length()
puts friends.empty?
puts friends.push('Creed')
puts friends.pop()
puts friends.unshift('Ryan')
puts friends.shift()
puts friends.insert(1, 'Pam')
puts friends.delete('Karen')
puts friends
puts friends.join(' - ')
puts friends[0..2]
# ----------------

# Hashes
# ----------------
# Keys must be unique
states = {
  :Pensilvania => 'PA',
  'New York' => 'NY',
  'Oregon' => 'OR'
}
# puts states
puts states[:Pensilvania] # Access by key
# ----------------

# Methods
# ----------------
def greetings (name = 'You', age = 25)
  # If you put a variable with #{}, it will become a string
  puts "Hello #{name}, you are #{age}"
end

# If you don't pass any arguments (greetings()), the default values will be used
# You can even write just greetings and it will work with the default values
greetings('Mike', 30)
# ----------------

# Return statement
# ----------------
def cube(num)
  # If you don't put return, the last line will be returned
  # When you put return, the function will end
  return num**3,5
end
# The first value will be returned
result = cube(3)[0]
puts result
# ----------------

# If statements
# ----------------

isClient = true
isPremium = true

if isClient and isPremium
  puts 'Welcome Premium Client'
elsif isClient and !isPremium
  puts 'Welcome Client'
else
  puts 'Welcome Guest'
end

balance = 100

if balance > 100
  puts 'You have a balance greater than 100'
elsif balance == 100
  puts 'You have a balance of 100'
else
  puts 'You have a balance less than 100'
end

# ----------------

# Comparison operators
# ----------------
# ==, !=, >, <, >=, <=
# and, or, not
Clara = {
  :age => 25,
  :gpa => 3.5,
  :balance => 100
}

Steven = {
  :age => 25,
  :gpa => 3.5,
  :balance => 260
}

if Clara[:balance] > Steven[:balance]
  puts 'Clara has more money than Steven'
elsif Clara[:balance] < Steven[:balance]
  puts 'Steven has more money than Clara'
else
  puts 'They have the same amount of money'
end

if Clara[:age] != Steven[:age]
  puts 'They are not the same age'
else
  puts 'They are the same age'
end

if Clara[:gpa] == Steven[:gpa]
  puts 'They have the same gpa'
elsif Clara[:gpa] > Steven[:gpa]
  puts 'Clara has a better gpa'
else
  puts 'Steven has a better gpa'
end

if Clara[:age] == 25 and Clara[:gpa] == 3.5
  puts 'Clara is 25 years old and has a 3.5 gpa'
else
  puts 'Clara is not 25 years old and does not have a 3.5 gpa'
end

if Steven[:age] == 25 or Steven[:gpa] == 3.5
  puts 'Steven is 25 years old or has a 3.5 gpa'
else
  puts 'Steven is not 25 years old or does not have a 3.5 gpa'
end

if not Clara[:balance] == 100
  puts 'Clara does not have a balance of 100'
else
  puts 'Clara has a balance of 100'
end
# ----------------

# Case statements
# ----------------
def get_day_name(day)
  day_name = ''
  case day
  when 'mon'
    day_name = 'Monday'
  when 'tue'
    day_name = 'Tuesday'
  when 'wed'
    day_name = 'Wednesday'
  when 'thu'
    day_name = 'Thursday'
  when 'fri'
    day_name = 'Friday'
  when 'sat'
    day_name = 'Saturday'
  when 'sun'
    day_name = 'Sunday'
  else
    day_name = 'Invalid abbreviation'
  end
  return day_name
end

puts get_day_name('mon')

# ----------------

# While loops
# ----------------
index = 1
while index <= 5
  puts index
  index += 1
end

# ----------------

# For and each loops
# ----------------
friends = ['Kevin', 'Karen', 'Oscar', 'Angela', 'Andy']
for friend in friends
  puts friend
end

for index in 0..5
  puts index
end

# Do is a word that can be used to replace {}
6.times do |index|
  puts index
end

6.times { |index| puts index }

puts [1,2,3].select(&:even?)

# Each is a method that can be used to iterate over a collection
[1, 2, 3].each { |n| puts "Current number is: #{n}" }

[1, 2, 3].each do |n|
  text = "Current number is: #{n}"
  puts text
end

my_hash = {min: 2, max: 5}
my_hash.each { |key, value| puts "k: #{key}, v: #{value}" }

[1, 2, 3, 4 ,5].drop(3).each { |i| puts i }

[1,2,3].each do |i|
  break if i == 3
  puts i
end

# Each with index lets you access the index of the element
# even if there is a hash
['a', 'b', 'c'].each_with_index { |el, i| puts i }

=begin
For vs each
for is used to iterate over a range of numbers
each is used to iterate over a collection of elements
Each is more idiomatic in Ruby
For is more idiomatic in other languages
Each is more flexible
Each is faster
In conclusion, use each over for
=end

# ----------------

# Exponent method
# ----------------
def pow(base_num, pow_num)
  result = 1
  pow_num.times do
    result *= base_num
  end
  return result
end

puts pow(2, 3)
# ----------------

# Recursion
# ----------------
# A method that calls itself
# Must have a base case to stop the recursion
def factorial(num)
  if num == 0
    return 1
  end
  return num * factorial(num - 1)
end

puts factorial(5)
# ----------------
