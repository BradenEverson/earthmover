# ArrowHead Transfer Protocol (AHTP)

The `Earthmover` agent must communicate with it's `Hivemind` server to plan, reason and advance its arbitrary goals. To do this, it must maintain a connection with this server and communicate in a session. The ArrowHead Transfer Protocol is designed with this goal in mind; being designed as a Cloud-Based transfer protocol where heavy amounts of data are sent up to a configured target, with some reaction falling back down. This document aims to describe the message types and protocol spec for AHTP:

## Protocol Steps
1) Initiation
2) Communication
3) Response

## Initiation
The `Hivemind` must be aware not only of the `agent`'s sorroundings and observations, but the physical body which will navigate this environment. Initiation of a session is where this context is supplied. 

**From Client**: Serialized URDF String describing Agent structure (must be URL encoded)
**Response**: Session ID as a UUID. This will be used in all future messages to notify what session we are actively in

### Example

```bash
GET https://{hivemind ip}:7878/initiation?urdf=%3Crobot%20name%3D%22simple_robot%22%3E%3Clink%20name%3D%22base_link%22%2F%3E%3C%2Frobot%3E
Response: 200 25c39361-02ad-4ee5-880d-ce0e39f7c7e9 
```

## Communication
Once a session is initiated, the server is ready to accept a websocket session with the `agent`. This is formally known as the communication stage, in which the `agent` will connect to the `session` and begin streaming it's peripheral readings to the `hivemind`. The different message types in this state are best described as Rust-style enums or TypeScript-style tagged unions(algebraic data types):

### Sending Messages

* **SEND**: Send relevant data as a tuple of 32 bit floating point numbers of unknown size. This allows for xyz coordinates to be registered, alongside any other relevant peripheral readings. 
    For example: An agent wishing to send x, y, z, thermistor, and light sensitivity data may look as follows:
    - `SEND: [[0.0, 0.5, 0.7, 1.3, 0.85],[0.2, 0.32, 7.6, 11.5, 0.0],[0.32, 5.4, 3.5, 9.0, 1.1]]`
* **GOAL**: A list of tuples of the form `(unsigned number, boolean)` that describes the indices of what reading values we want to maximize/minimize along with a boolean for whether maximize is true. In other words, true for maximize value, false for minimize.
    For example: Using the previous example again, if we wanted to maximize the thermisor and minimize the light values we would send:
    - `GOAL: [(3, true), (4, false)]`
    - Note: Any reoccuring indices is considered an error

### Receiving Messages

* **INSTR**: An instruction set sent from the `hivemind` to the `agent`. This describes the actions necessary to get closer to completing the submitted goal.
    - The **INSTR** format goes as follows, and takes up 16 bytes per message:
        - **node:** The ID of the peripheral output node this instruction targets (4 bytes)
        - **lasts_for_ms**: The time in milliseconds that this instruction should last for (4 bytes)
        - **instructions**: 4 optional bytes representing information for the designated output node. These bytes can be interpretted differently based on what output is being communicated with, making this very abstract
    - An **INSTR** could be a list of several **INSTR** as well. This allows for chained movements
    - Example: if the `hivemind` computed the way to get closer to a designated goal was to move servo `2` `180` degrees, the **INSTR** could be something as follows:
        - `INSTR: {node: 2, lasts_for_ms: 1000, instructions: [180, None, None, None]}`