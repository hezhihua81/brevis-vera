## ADDED Requirements

### Requirement: Crop image transformation
The system SHALL support cropping images to a specified region using the image crate.

#### Scenario: Successful crop
- **WHEN** user specifies crop parameters in format "x,y,width,height"
- **THEN** system produces a cropped image containing only the specified region

#### Scenario: Invalid crop parameters
- **WHEN** crop parameters are not in correct format (must be 4 comma-separated values)
- **THEN** system rejects the operation and outputs an error

#### Scenario: Input file not found
- **WHEN** specified input file does not exist
- **THEN** system returns error indicating input file not found

#### Scenario: Image load failure
- **WHEN** input file cannot be loaded as an image
- **THEN** system returns error indicating image load failure

### Requirement: Resize transformation
The system SHALL support resizing images to specified dimensions using Lanczos3 filter.

#### Scenario: Successful resize
- **WHEN** user specifies new dimensions in format "width,height"
- **THEN** system produces a resized image matching specified dimensions

#### Scenario: Invalid resize parameters
- **WHEN** resize parameters are not in correct format (must be 2 comma-separated values)
- **THEN** system rejects the operation and outputs an error

### Requirement: Brightness adjustment
The system SHALL support adjusting image brightness with values from -100 to 100.

#### Scenario: Successful brightness adjustment
- **WHEN** user specifies brightness value
- **THEN** system adjusts all RGB pixel values by the specified amount (scaled by 2.55)

#### Scenario: Zero brightness adjustment
- **WHEN** brightness value is 0
- **THEN** system skips brightness adjustment

### Requirement: Save edited image
The system SHALL save the edited image to the specified output path.

#### Scenario: Save failure
- **WHEN** system cannot save to the specified output path
- **THEN** system returns error indicating save failure

### Requirement: Record transformation parameters
The system SHALL record all transformation parameters applied during editing for ZK proof generation.

#### Scenario: Parameter recording
- **WHEN** image is transformed
- **THEN** system returns TransformParams struct with crop, resize, and brightness fields

### Requirement: Combined transformations
The system SHALL support applying multiple transformations in sequence (crop -> resize -> brightness).

#### Scenario: Multiple transformations
- **WHEN** user specifies multiple transformation parameters
- **THEN** system applies transformations in order and returns combined parameters
