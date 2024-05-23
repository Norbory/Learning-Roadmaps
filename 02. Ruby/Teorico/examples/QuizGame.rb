class Car
  attr_accessor :make, :model, :year, :color, :price
  def initialize (make, model, year, color, price)
    @make = make
    @model = model
    @year = year
    @color = color
    @price = price
  end

  def car_details
    "This is a #{@year} #{@make} #{@model} with #{@color} color."
  end


end

car1 = Car.new("Toyota", "Corolla", 2010, "Red", 15000)
car2 = Car.new("Honda", "Civic", 2015, "Blue", 20000)
car3 = Car.new("Ford", "Fiesta", 2018, "Black", 18000)

def quiz_game
  puts "Welcome to the Car Quiz Game!"
  puts "Answer the following questions to win a car!"
  puts "Choose the correct option to answer the questions."
  puts "Choose a car"
  puts "1. Toyota"
  puts "2. Honda"
  puts "3. Ford"
  car_choice = gets.chomp.to_i
  case car_choice
    when 1
      puts "Guess the model of the car"
      puts "1. Corolla"
      puts "2. Camry"
      puts "3. Prius"
      model_choice = gets.chomp.to_i
      if model_choice == 1
        puts "Correct! You win a Toyota Corolla!"
      else
        puts "Incorrect! You lose!"
      end
    when 2
      puts "Guess the year of the car"
      puts "1. 2010"
      puts "2. 2015"
      puts "3. 2018"
      year_choice = gets.chomp.to_i
      if year_choice == 2
        puts "Correct! You win a Honda Civic!"
      else
        puts "Incorrect! You lose!"
      end
    when 3
      puts "Guess the color of the car"
      puts "1. Red"
      puts "2. Blue"
      puts "3. Black"
      color_choice = gets.chomp.to_i
      if color_choice == 3
        puts "Correct! You win a Ford Fiesta!"
      else
        puts "Incorrect! You lose!"
      end
    else
      puts "Invalid choice! Choose a valid option."
      quiz_game()
  end
end

quiz_game()
