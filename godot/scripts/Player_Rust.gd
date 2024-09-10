extends Player

signal hit

# 因为在Rust binding之中对接收signal的支持并不是很好，
# 所以有关于接收signal的逻辑全部都放到gdscript之中去做
func _on_MobDetector_body_entered(_body):
	die()
