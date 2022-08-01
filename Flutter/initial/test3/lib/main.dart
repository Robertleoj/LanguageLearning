import 'package:flutter/material.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {


    return Container(
      width: 100,
      height: 100,
      color: Colors.red,
      child: const Center(
        child: Text(
          'Hello',
          textDirection: TextDirection.ltr
        )
      )
    );


  }
}