from django.urls import path

from salvin import views

urlpatterns = [
    path("salvin/", views.temp_here, name="temp_here"),
    path("salvin/discover", views.temp_somewhere, name="temp_somewhere")
]
