magic_word = 'ruby'
guess = ''
word = '____'

=begin
There is important to note that the method clues is defined before the while loop
This is because the method is being called inside the loop
If the method was defined after the loop, the program would not be able to find the method
due to the scope of the method
=end

def clues(letters, magic_word, word)
  letters.each do |letter|
    if magic_word.include?(letter)
      puts "There is a #{letter} in the magic word"
      word[magic_word.index(letter)] = letter
    else
      puts "There is no #{letter} in the magic word"
    end
  end
  puts word
end

while guess != magic_word
  puts 'Choose 3 random letters:'
  letters = Array.new(3) { gets.chomp() }
  clues(letters, magic_word, word)
  puts 'You want to guess the magic word? yes or no'
  choice = gets.chomp()
  if choice == 'no'
    puts 'Try again!'
  elsif choice == 'yes'
    puts 'What is your guess?'
    guess = gets.chomp()
    if guess == magic_word
      puts 'You win!'
    else
      puts 'Try again!'
    end
  end
end

puts 'You win!'
