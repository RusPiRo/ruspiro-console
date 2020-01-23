# Changelog
## :apple: v0.3.1
  - ### :bulb: Features
    
  - ### :detective: Fixes
    - writing to the console is no longer a blocking operation. This allows console outputs from within
    Interrupt handlers without possible deadlocks. However, concurrent console writes may lead to "garbage"
    on the console output (terminal)
    
  - ### :wrench: Maintenance
    