public class Hello {
    public static void main(String[] args) {
        // Introducción a Java y tipos de datos
        String saludar = "Hola putito";
        System.out.println(saludar);
        System.out.println(saludar.toUpperCase());

        int numero = 5;
        System.out.println("El número es: " + numero);

        boolean valor = true;
        int numero2;
        if (valor) {
            // No puedes redeclarar la variable 'numero' en el mismo ámbito
            //int numero = 10;
            numero2 = 10;
            System.out.println("El valor es verdadero y el número2 es: " + numero2);
        } else {
            System.out.println("El valor es falso");
        }

        // Variable dinamica
        var decimal = 5.99;
        System.out.println("El número decimal es: " + decimal);
    }
}