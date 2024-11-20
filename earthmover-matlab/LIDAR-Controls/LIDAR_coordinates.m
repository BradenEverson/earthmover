function [coordinates] = LIDAR_coordinates(rollAngle,pitchAngle,D,lidarAngle)
% LIDAR_coordinates uses angles from gimbal and LIDAR, and measured
% distance to calculate an x,y,z coordinate set
%
%% Function arguments:
%
% INPUT arguments:
% rollAngle - roll angle [degrees]
% pitchAngle - pitch angle around the roll vector [degrees]
% D - distance measured by LIDAR [in]
% lidarAngle - LIDAR angle clockwise from pitch vector [degrees]
%
% OUTPUT arguments:
% coordinates - x,y,z coordinates of point as a row vector

%% Header
% Date Written: 11/19/2024
% Date Modified: N/A
% Written by: Matt Griesbach

%% Code Statements:

% angles
thetaR = deg2rad(rollAngle); % Roll angle [radians]
thetaP = deg2rad(pitchAngle); % Pitch angle around R [radians]
thetaL = deg2rad(-1*lidarAngle); % LIDAR angle counter clockwise from P [radians]

% LIDAR offset
% oP = -13.2842; % offset along P [mm] (-0.523 in)
% oN = 40.8432; % offset along Nu [mm] (1.608 in)
% oR = -0.1016; % offset along R [mm] (-0.004 in)

oP = -0.523; % offset along P [in]
oN = 1.608; % offset along Nu [in]
oR = -0.004; % offset along R [in]

%component values
% R
Rx = 0;
Ry = cos(thetaR);
Rz = sin(thetaR);
% P
Pperp = sin(thetaP);
Px = cos(thetaP);
Py = Pperp*sin(-1*thetaR);
Pz = Pperp*cos(thetaR);

% Cross product results
% Nu = R x P
Nux = Py*Rz-Pz*Ry;
Nuy = Pz*Rx-Px*Rz;
Nuz = Px*Ry-Py*Rx;

% Nl magnitue
% Nl = L*P*sin(thetaL) = sin(thetaL)
Nl = sin(thetaL);
% Nl vector from unit vector Nu and scalar Nl
Nlx = Nux*Nl;
Nly = Nuy*Nl;
Nlz = Nuz*Nl;

if((thetaL~=0)&&(thetaL~=pi)) % cross product only works if thetaL is 
                            % not parallel to P, thetaL is not 0 or 180
                            % degrees
    % solve for vector L
    Lx = (Px*cos(thetaL)-Py*Nlz+Pz*Nly)/(Px^2+Py^2+Pz^2);
    Ly = (Py*cos(thetaL)-Pz*Nlx+Px*Nlz)/(Px^2+Py^2+Pz^2);
    Lz = (Pz*cos(thetaL)-Px*Nly+Py*Nlx)/(Px^2+Py^2+Pz^2);
elseif(thetaL==0) % if thetaL is 0, L is equal to P
    Lx = Px;
    Ly = Py;
    Lz = Pz;
else % if thetaL is 180 degrees, L is equal to -P
    Lx = -Px;
    Ly = -Py;
    Lz = -Pz;
end

% solve for Vector D and add offset, final answer
Dx = D*Lx + oP*Px + oR*Rx + oN*Nux;
Dy = D*Ly + oP*Py + oR*Ry + oN*Nuy;
Dz = D*Lz + oP*Pz + oR*Rz + oN*Nuz;

coordinates = [Dx,Dy,Dz];
end