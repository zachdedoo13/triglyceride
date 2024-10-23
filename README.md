# Trigyceride
##### named arfter the tree lick structure of a Trigyceride molocule, trust me it makes sence


## Overview
used to monitor the perfomance of singlethreaded, event loop based applications
> uses egui for a tree est perfomance overview and detailed perfomance graphs

## Usecases
- Eframe apps can easaly be profiled and displayed
- apps that do not use winit or any display can use the seperate window function to spawn the ui on anuther thread, // multuple winit windows at onece on diffrent threads is not supported, help is welcomed for this feature 

## Limitations 
- currently only supports windows, will fix at some point
- ui's kinda bad, working on a costom solution rarther then useing egui plots bar graph (hacky)

## Benifits
- diognose bugs and bottlenecks in an intuitive way
- add cool looking perfomance statictics to you app to make it feel cooler wist provideing no tangable benifit 
