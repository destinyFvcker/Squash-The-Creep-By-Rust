extends CharacterBody3D

# Emitted when the player jumped on the mob.
signal squashed

func _on_visible_on_screen_notifier_screen_exited():
	queue_free()
