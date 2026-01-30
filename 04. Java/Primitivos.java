public class Primitivos {

    // Puedo declarar variables fuera del main, pero dentro de la clase
    static int variableDeClase = 10;

    public static void main(String[] args) {
        // Tipos de datos primitivos en Java
        // Byte: 8 bits - rango de -128 a 127
        byte numeroByte = 7;
        System.out.println("Valor maximo de byte: " + Byte.MAX_VALUE);
        System.out.println("Valor minimo de byte: " + Byte.MIN_VALUE);

        // Short: 16 bits - rango de -32,768 a 32,767
        short numeroShort = 30000;

        // Int: 32 bits - rango de -2,147,483,648 a 2,147,483,647
        int numeroInt = 2000000000;

        // Long: 64 bits - rango de -9,223,372,036,854,775,808 a 9,223,372,036,854,775,807
        long numeroLong = 9000000000L;

        System.out.println(" Los valores de los enteros son: ");
        System.out.println("Byte: " + numeroByte);
        System.out.println("Short: " + numeroShort);
        System.out.println("Int: " + numeroInt);
        System.out.println("Long: " + numeroLong);

        // Float: 32 bits - números decimales de precisión simple
        float numeroFloat = 5.75f;
        float conNotacionCientifica = 3.4e+3f;
        float conNotacionCientificaNegativa = 3.4e-3f;
        System.out.println("El maximo valor de float es: " + Float.MAX_VALUE);
        System.out.println("El minimo valor de float es: " + Float.MIN_VALUE);

        // Double: 64 bits - números decimales de precisión doble
        double numeroDouble = 19.99e+39d;
        System.out.println("El maximo valor de double es: " + Double.MAX_VALUE);
        System.out.println("El minimo valor de double es: " + Double.MIN_VALUE);

        System.out.println(" Los valores de los decimales son: ");
        System.out.println("Float: " + numeroFloat);
        System.out.println("Float (notación científica): " + conNotacionCientifica);
        System.out.println("Float (notación científica negativa): " + conNotacionCientificaNegativa);
        System.out.println("Double: " + numeroDouble);
        // Si no colocamos la "f" al numero decimal, Java lo toma como double por defecto

        System.out.println(" El valor de la variable de clase es: " + variableDeClase);

        // Char: 16 bits - representa un solo carácter Unicode
        char caracter = '\u0040';
        char decimal = 64;
        System.out.println("Char: " + caracter);
        System.out.println("Char (decimal): " + decimal);

        // Boolean: representa un valor de verdad (true o false)
        boolean verdadero = true;
        System.out.println("Boolean: " + verdadero);
    }
}
