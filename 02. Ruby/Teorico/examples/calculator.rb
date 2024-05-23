# Calculator program
def operacion (operacion, num1, num2)
  case operacion
    when "1"
      return num1 + num2
    when "2"
      return num1 - num2
    when "3"
      return num1 * num2
    when "4"
      return num1 / num2
    else
      return "Operación no válida"
    end
end

print "Introduce el primer número:"
num1 = gets.chomp.to_f
puts "Introduce el segundo número:"
num2 = gets.chomp.to_f
puts "Introduce la operación a realizar:"
puts "1. Suma"
puts "2. Resta"
puts "3. Multiplicación"
puts "4. División"
operacion = gets.chomp

puts "El resultado es: #{operacion(operacion, num1, num2)}"
