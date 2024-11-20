%LIDAR_code.m
% Date Written: 11/19/2024
% Date Modified: N/A
% Written By: Matt Griesbach
%
% uses  functions to control the motion of the LIDAR, convert result to 
% x,y,z coordinates, and simulate the output
clear;clc;close all;

%% Initialization Variables
maxAngle = 25; % maximum angle from horizontal for the gimbal [degrees]
resolution = 70; % number of steps per cycle
period = 1; % time for one full cycle of the gimbal [sec]

%% Code Statements:
% set roll motor to 0 degrees
% set pitch motor to 0 degrees
delay = period/((resolution+3)^2) % delay time for plotting

angleSet = motorPositions(maxAngle,resolution); % creates a matrix of the 
    % gimbal motor angles for one full cycle

figure; % open plot figure
view(161.37,30) % set view angle for the plot

for i=2:(resolution+3) % cycle through the matrix of angle data. 
    
    tic % start clock
    
    % convert next angle to steps and directions
    roll_motor_steps = DegToStep(angleSet((i-1),1),angleSet(i,1));
    pitch_motor_steps = DegToStep(angleSet((i-1),2),angleSet(i,2));
    % move roll motor
    % move pitch motor

    % collect LIDAR angles and distances as a (n X 2) matrix of
        % (dist,angle)
    b = size(angleSet,1); % get the number of rows in angleSet
    LIDAR_data = [linspace(10,10,b)',(angleSet(:,3))]; % LIDAR dataset 
        % to substitute actual readings.
        % use [10,-1*angleSet(i,3)] to only get peak z values.
        % use [randi([10,14],1,b)',(angleSet(:,3))] for random size.
        % use [linspace(10,10,b)',(angleSet(:,3))] for fixed distance.
        % use [10+2*sind(9*angleSet(:,3)),(angleSet(:,3))] for star shape.
    
    n = size(LIDAR_data,1); % get the number of rows in LIDAR_data set 

    % get coordinates for all calculated angles
    for j=1:n
        k = j + n*(i-1); % counter variable for inner for loop

        % calculate and store coordinates in matrix
        coordinateData(k,:) = LIDAR_coordinates(angleSet(i,1),angleSet(i,2),LIDAR_data(j,1),LIDAR_data(j,2));

        % plotting moving distance vector output for MATLAB simulation
        % cla % clear plot. uncomment this to display a single laser
            % line, comment out to show disk forming
        hold on
        plot3([0,coordinateData(k,1)],[0,coordinateData(k,2)],[0,coordinateData(k,3)],'r-^');
        % plot3(coordinateData(1:k,1),coordinateData(1:k,2),coordinateData(1:k,3),'b.'); % uncomment to live plot coordinates
        grid on;
        xlabel('X axis'), ylabel('Y axis'), zlabel('Z axis')
        xlim([-20 20]);ylim([-20 20]);zlim([-20 20]);
        %set(gca,'CameraPosition',[1 2 3]);

        % pause(period/(n^3)) % pause between each point. uncomment to see
            % each line added one at a time
    end
    clock = toc % read clock
    pause(period/(n)) % pause between each point
    LIDAR_data = zeros(size(LIDAR_data)); % reset matrix to zeros to speed 
        % up calculatiuon time
    cla % clear LIDAR disk

end
% plot full coordinate set
[caz,cel] = view; % get view coordinates for plot
hold on
plot3(coordinateData(:,1),coordinateData(:,2),coordinateData(:,3),'b.');
grid on;
xlabel('X axis'), ylabel('Y axis'), zlabel('Z axis')
xlim([-20 20]);ylim([-20 20]);zlim([-20 20]);
set(gca,'CameraPosition',[1 2 3]);
view(161.37,9)
