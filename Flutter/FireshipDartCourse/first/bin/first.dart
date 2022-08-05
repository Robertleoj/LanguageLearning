

class Animal {
  late final int age;

  Animal(this.age);

  String getAgeString(){
    return "The age is ${this.age}";
  }

  uhh() {
    print("No return type");
  }

}

abstract class CInterface {
  String? a;
  int? b;

  summarize();
}

class CHard extends CInterface {

  CHard(String a, int b){
    this.a = a;
    this.b = b;
  }

  @override
  summarize(){
    print("$a $b");
  }
}

void main(List<String> arguments) {

  CInterface c = CHard("shit", 788);
  c.summarize();

  // Animal a = Animal(7);
  // print(a.getAgeString());

  // bonkIt(Function b) {
  //   b();
  // }

  // bonkIt( () {
  //   print("I'm so anonymous");
  // } );
  // List<int> l = [1, 3, 4];

  // print(l);
  // print(l.sublist(1, 2));

  // print(List.filled(50, "bla"));

}
