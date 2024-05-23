// See https://aka.ms/new-console-template for more information
using System;

namespace CSharpHelloWorld
{
    class HelloWorld
    {
        static void Main(string[] args)
        {
            // Display the string with a variable
            string name = "John";
            Console.WriteLine("Hello, " + name);
            // name = "Eric";
            // Console.WriteLine("Hello, " + name);
            // name = Console.ReadLine();
            // if (name == null)
            // {
            //     Console.WriteLine("Name is null");
            // } else
            // {
            //     Console.WriteLine("Hello, " + name);
            // }

            // Display the INT with a variable
            int age = 18;
            int futureAge = age + 10;
            Console.WriteLine("In 10 years, you will be " + futureAge);

            // Display the double with a variable
            double price = 19.95;
            double tax = 0.07;
            double total = price + price * tax;
            Console.WriteLine("Total: " + total);

            // Display the float with a variable
            float numberf = 1.23456789f;
            Console.WriteLine("Float: " + numberf);

            // Display the decimal with a variable
            decimal numberd = 1.23456789m;
            Console.WriteLine("Decimal: " + numberd);

            // Display the dynamic with a variable (Not recommended to use)
            dynamic data = 1;
            Console.WriteLine("Dynamic: " + data);
            data = "Hello";
            Console.WriteLine("Dynamic: " + data);

            // Display the boolean with a variable
            bool isTrue = true;
            Console.WriteLine("Boolean: " + isTrue);
            isTrue = !isTrue; // Negation
            Console.WriteLine("Boolean: " + isTrue);

            // Display the char with a variable
            char letter = 'A';
            Console.WriteLine("Char: " + letter);

            // Display the constant with a variable
            // Declared with type and value
            const double PI = 3.14159;
            Console.WriteLine("PI: " + PI);
            // Not allowed to change the value of a constant
            // PI = 3.14;  Error

            // Display the var with a variable
            var number = "Hello";
            // Inferred as a string
            // Not allowed to change the type of a var even though it is inferred
            // Not equivalent to dynamic
            Console.WriteLine("Var: " + number);

            // Display multiple variables without + operator
            Console.WriteLine($"Mis variables {number}, {letter}, {isTrue}, {data}, {numberd}, {numberf}, {total}, {futureAge}, {name}");

            // Estrucuturas de control

            // Arreglos
            var myArray = new string[] {"Angelo", "Mandros", "Norbory"};
            Console.WriteLine(myArray[0]);

            // No se puede cambiar el tamaño de un arreglo
            // No puede contener elementos de diferentes tipos
            // myArray[3] = "27"; // Error
            // Console.WriteLine(myArray[3]); // Error

            // Dictionary
            var myDictionary = new Dictionary<string, int>
            {
                {"Angelo", 27},
                {"Juan", 25},
                {"Antonio", 26}
            };
            Console.WriteLine(myDictionary["Angelo"]);

            // Sets
            // Estructura de datos que no permite elementos duplicados por lo que no se imprime "Angelo" dos veces
            // Estrucutra de datos que no permite elementos ordenados
            var mySet = new HashSet<string> {"Angelo", "Juan", "Antonio", "Angelo"};
            Console.WriteLine(mySet);

            // Tuple
            // Estructura de datos que permite almacenar varios elementos de diferentes tipos
            // No se puede cambiar el tamaño de un Tuple
            var myTuple = Tuple.Create(1, "Angelo", 27);
            Console.WriteLine(myTuple.Item2);

            // Bucles
            // For
            for (int i = 0; i < 10; i++)
            {
                Console.WriteLine(i);
            }

            // foreach
            foreach (var item in myArray)
            {
                Console.WriteLine(item);
            }
            foreach (var item in myDictionary)
            {
                Console.WriteLine(item.Key + " " + item.Value);
            }
            foreach (var item in mySet)
            {
                Console.WriteLine(item);
            }
            // No se puede hacer un foreach de un Tuple
            // foreach (var item in myTuple)
            // {
            //     Console.WriteLine(item);
            // }

            // Flujo de control
            // If
            if (age > 18 && isTrue)
            {
                Console.WriteLine("Mayor de edad");
            }
            else if (age == 18 || isTrue)
            {
                Console.WriteLine("Tiene 18 años");
            }
            else
            {
                Console.WriteLine("Menor de edad");
            }
            
            // Funciones
            myMethod();
            myMethod();
            myMethod();
            Console.WriteLine(myMethodWithReturn(10));

            // Clases
            var myClass = new myClass("Angelo");
            myClass.myName = "Mandros";
            Console.WriteLine(myClass.myName);
        }

        static void myMethod()
        {
            // No se puede acceder a variables locales de un método
            var name = "Angelo";
            Console.WriteLine(name);
        }

        static int myMethodWithReturn(int param)
        {
            // No se puede acceder a variables locales de un método
            return 10 + param;
        }
    
        // Al colocar public se puede acceder a la clase desde cualquier parte del código
        class myClass 
        {
            // Getters and Setters (Propiedades)
            // permite acceder a las variables de la clase
            public string myName { get; set; }

            // Constructor
            public myClass(string myName)
            {
                // Al colocar this señala que se está refiriendo a la variable de la clase
                this.myName = myName;
            }
        }
    }
}



