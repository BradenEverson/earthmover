# Use an official Python image from the Docker Hub
FROM python:3.10

# Set the working directory in the container
WORKDIR /app

# Install Jupyter and any other dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Copy your notebook files into the container (optional, if you have existing notebooks)
# COPY . .

# Expose the port for Jupyter
EXPOSE 8888

# Command to run Jupyter Notebook
CMD ["jupyter", "notebook", "--ip=0.0.0.0", "--port=8888", "--allow-root", "--no-browser"]
