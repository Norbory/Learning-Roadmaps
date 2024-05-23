# Classes in Ruby
# Classes are a way to bundle data and behavior together
class Book
  attr_accessor :title, :author, :pages
  # Constructor method
  def initialize(title, author, pages)
    @title = title
    @author = author
    @pages = pages
  end

  # Methods
  def big_book
    if @pages > 200
      return "This is a big book!"
    else
      return "This is a small book!"
    end
  end
end

# Inheritance
class Magazine < Book
  attr_accessor :issue
  def initialize(title, author, pages, issue)
    super(title, author, pages)
    @issue = issue
  end

  # Allows the child class to override the parent class method
  def big_book
    if @pages > 50
      return "This is a big magazine!"
    else
      return "This is a small magazine!"
    end
  end
end

# Create a new book object
# Objects are instances of classes
# Assign values to the attributes of the book object
# book1 = Book.new()
# book1.title = "Harry Potter"
# book1.author = "JK Rowling"
# book1.pages = 400

# book2 = Book.new()
# book2.title = "Lord of the Rings"
# book2.author = "Tolkien"
# book2.pages = 500

# To initialize the object with values
book1 = Book.new("The Alchemist", "Paulo Coelho", 200)
book2 = Book.new("The Power of Now", "Eckhart Tolle", 300)

puts book1.big_book
# puts book2.big_book

mag1 = Magazine.new("Time", "Time Inc", 100, "July 2021")
puts mag1.title
puts mag1.issue

# Override the parent class method
puts mag1.big_book()
