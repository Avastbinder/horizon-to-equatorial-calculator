# Overview

A Horizontal to Equatorial conversion tool written in Rust. 

Useful for Astronomy, such as getting the position of an object from the sky and converting those coordinates to Equatorial to find out what that object is.

There seems to be a lack of information online regarding this type of conversion, so I hope this will be helpful to anyone who wants to do this type of conversion.

[Software Demo Video](https://youtu.be/N5IOR7gEAng)

# Development Environment

Written on Visual Studio Code, and a very useful tool in the making of this application was [Stellarium](https://stellarium.org/), which allowed me to test if my calculations were accurate.

Written in Rust, and heavily uses the Iced crate which handles the user interface.

# Useful Websites

- [Positional Astronomy: Conversion between horizontal and equatorial systems](https://sceweb.sce.uhcl.edu/helm/WEB-Positional%20Astronomy/Tutorial/Conversion/Conversion.html)
- [MatLab program that performs this calculation](https://www.mathworks.com/matlabcentral/fileexchange/24581-convert-azimuth-and-elevation-to-right-ascension-and-declination)

# Future Work

- Make input fields easier to understand, maybe add notes to clarify what needs to be entered
- Make more accurate (decently accurate at this moment, but could always be better)
- Automate some of the input parameters, for example the latitude and longitude of observation
