# PbrEngine

A very basic CPU raytracing engine.

I wrote this in Rust following the awesome C++ tutorial https://raytracing.github.io/books/RayTracingInOneWeekend.html.


Unlike the tutorial, this engine is real time, in the sense that you can move the camera with mouse and keyboard and that the json scene file is hot reloaded.
Unfortunately, being this engine extremely simple and CPU based, the framerate is very low unless the scene is very simple or your CPU very beefy.
