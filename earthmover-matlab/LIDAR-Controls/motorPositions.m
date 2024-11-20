function [angleData] = motorPositions(maxAngle,n)
% motorPositions uses a max angle and time value to rotate the gimbal mount
% maintaining a constant max angle vrom horizontal that rotates about a
% verical axis
% Outputs pitch and roll angles for the gimbal mount. The number of steps 
% and direction for each motor will need to be calculated from the output 
% of this function based on previously stored positions.
%% Function arguments
%
% INPUT arguments:
% maxAngle - Maximum angle from horizontal [degrees]
% n - Number of steps per cycle (step resolution)
%
% OUTPUT arguments:
% angleData - A (n+1) X 3 matrix containing the roll, pitch, and peak angles
%               for one full cycle in form [roll,pitch,peak] [degrees]

%% Header
% Date Written: 11/16/2024
% Date Modified: N/A
% Written By: Matt Griesbach

%% Code Statements:
% intializing matrices with zeros speeds up calculation time
angles = zeros(n+1,1); % create a vertical array of zeros that starts at position 1
                           % and cycles back to position 1.
angleData = zeros(n+3,2); % create a matrix of zeros
for i=1:(n+1)
    angles(i) = 360/n*(i-1);
end

% angles
    % a 0 is added before and after each set so initial and ending positions
    % will be horizontal.
angleData(:,1) = [0;maxAngle*sin(angles/180*pi);0]; % roll angle [degrees]. Converted to radians for calculation
angleData(:,2) = [0;maxAngle*cos(angles/180*pi);0]; % pitch angle around R  [degrees]. Converted to radians for calculation
angleData(:,3) = [0;angles;0]; % angle of peak from horizontal
end


