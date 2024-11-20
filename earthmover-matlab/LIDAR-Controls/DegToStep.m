function [stepData] = DegToStep(prevAngle,newAngle)
% DegToStep converts the change in angle to the number of steps and motor
% direction.
%
%% Function arguments
%
% INPUT arguments:
% prevAngle - starting position angle
% newAngle - ending position angle
%
% OUTPUT arguments:
% stepData - number of stpes the motor needs to take, and direction motor 
%           will rotate, 1 is CW, -1 is CCW

%% Header
% Date Written: 11/19/2024
% Date Modified: N/A
% Written by: Matt Griesbach

%% Code Statements:
degPerStep = 360/4096; % degrees per full step

delta_A = prevAngle - newAngle; % change in angle
steps = abs(delta_A)/degPerStep; % convert degrees to steps
direction = delta_A/abs(delta_A); % calculate direction value
stepData = [steps,direction];
end