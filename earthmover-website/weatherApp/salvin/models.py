from django.db import models


# Create your models here.
class Worldcities(models.Model):
    city = models.TextField(blank=True, null=True)
    lat = models.TextField(blank=True, null=True)  # This field type is a guess.
    lng = models.TextField(blank=True, null=True)  # This field type is a guess.
    id = models.TextField(blank=True, primary_key=True)

    class Meta:
        managed = False
        db_table = 'worldcities'