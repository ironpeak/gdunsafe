#[macro_export]
macro_rules! get_tree {
    ($owner:tt) => {
        unsafe { $owner.get_tree().unwrap().assume_safe() }
    };
}

#[macro_export]
macro_rules! get_root {
    ($owner:tt) => {
        unsafe {
            $owner
                .get_tree()
                .unwrap()
                .assume_safe()
                .root()
                .unwrap()
                .assume_safe()
        }
    };
}

#[macro_export]
macro_rules! get_node {
    ($node:tt, $path:tt) => {
        unsafe { $node.get_node($path).unwrap().assume_safe() }
    };
}

#[macro_export]
macro_rules! __impl_get_node_type {
    ($name:ident, $type:tt) => {
        #[macro_export]
        macro_rules! $name {
            ($node:tt, $path:tt) => {
                unsafe {
                    $node
                        .get_node($path)
                        .unwrap()
                        .assume_safe()
                        .cast::<$type>()
                        .unwrap()
                }
            };
        }
    };
}

__impl_get_node_type!(get_node_accept_dialog, AcceptDialog);
__impl_get_node_type!(get_node_animated_sprite, AnimatedSprite);
__impl_get_node_type!(get_node_animation_player, AnimationPlayer);
__impl_get_node_type!(get_node_animation_tree, AnimationTree);
__impl_get_node_type!(get_node_animation_tree_player, AnimationTreePlayer);
__impl_get_node_type!(get_node_area, Area);
__impl_get_node_type!(get_node_area_2d, Area2D);
__impl_get_node_type!(get_node_arvr_anchor, ARVRAnchor);
__impl_get_node_type!(get_node_arvr_, ARVRCamera);
__impl_get_node_type!(get_node_arvr_, ARVRController);
__impl_get_node_type!(get_node_arvr_, ARVROrigin);
__impl_get_node_type!(get_node_, AspectRatioContainer);
__impl_get_node_type!(get_node_audio_stream_player, AudioStreamPlayer);
__impl_get_node_type!(get_node_audio_stream_player, AudioStreamPlayer2D);
__impl_get_node_type!(get_node_audio_stream_player, AudioStreamPlayer3D);
__impl_get_node_type!(get_node_, BackBufferCopy);
__impl_get_node_type!(get_node_, BaseButton);
__impl_get_node_type!(get_node_, Bone2D);
__impl_get_node_type!(get_node_, BoneAttachment);
__impl_get_node_type!(get_node_, BoxContainer);
__impl_get_node_type!(get_node_, Button);
__impl_get_node_type!(get_node_, Camera);
__impl_get_node_type!(get_node_, Camera2D);
__impl_get_node_type!(get_node_, CanvasItem);
__impl_get_node_type!(get_node_, CanvasLayer);
__impl_get_node_type!(get_node_, CanvasModulate);
__impl_get_node_type!(get_node_, CenterContainer);
__impl_get_node_type!(get_node_, CheckBox);
__impl_get_node_type!(get_node_, CheckButton);
__impl_get_node_type!(get_node_, ClippedCamera);
__impl_get_node_type!(get_node_collision_, CollisionObject);
__impl_get_node_type!(get_node_collision_, CollisionObject2D);
__impl_get_node_type!(get_node_collision_, CollisionPolygon);
__impl_get_node_type!(get_node_collision_, CollisionPolygon2D);
__impl_get_node_type!(get_node_collision_, CollisionShape);
__impl_get_node_type!(get_node_collision_, CollisionShape2D);
__impl_get_node_type!(get_node_, ColorPicker);
__impl_get_node_type!(get_node_, ColorPickerButton);
__impl_get_node_type!(get_node_, ColorRect);
__impl_get_node_type!(get_node_, ConeTwistJoint);
__impl_get_node_type!(get_node_, ConfirmationDialog);
__impl_get_node_type!(get_node_, Container);
__impl_get_node_type!(get_node_, Control);
__impl_get_node_type!(get_node_, CPUParticles2D);
__impl_get_node_type!(get_node_, CullInstance);
__impl_get_node_type!(get_node_, DampedSpringJoint2D);
__impl_get_node_type!(get_node_, EditorFileDialog);
__impl_get_node_type!(get_node_, EditorFileSystem);
__impl_get_node_type!(get_node_, EditorInspector);
__impl_get_node_type!(get_node_, EditorInterface);
__impl_get_node_type!(get_node_, EditorPlugin);
__impl_get_node_type!(get_node_, EditorProperty);
__impl_get_node_type!(get_node_, EditorResourcePreview);
__impl_get_node_type!(get_node_, EditorScriptPicker);
__impl_get_node_type!(get_node_, EditorSpinSlider);
__impl_get_node_type!(get_node_, FileDialog);
__impl_get_node_type!(get_node_, FileSystemDock);
__impl_get_node_type!(get_node_, Generic6DOFJoint);
__impl_get_node_type!(get_node_, GraphEdit);
__impl_get_node_type!(get_node_, GraphNode);
__impl_get_node_type!(get_node_, GridContainer);
__impl_get_node_type!(get_node_, GridMap);
__impl_get_node_type!(get_node_, GrooveJoint2D);
__impl_get_node_type!(get_node_, HBoxContainer);
__impl_get_node_type!(get_node_, HingeJoint);
__impl_get_node_type!(get_node_, HScrollBar);
__impl_get_node_type!(get_node_, HSeparator);
__impl_get_node_type!(get_node_, HSlider);
__impl_get_node_type!(get_node_, HSplitContainer);
__impl_get_node_type!(get_node_, HTTPRequest);
__impl_get_node_type!(get_node_, InstancePlaceholder);
__impl_get_node_type!(get_node_, InterpolatedCamera);
__impl_get_node_type!(get_node_, ItemList);
__impl_get_node_type!(get_node_, Joint);
__impl_get_node_type!(get_node_, Joint2D);
__impl_get_node_type!(get_node_, KinematicBody);
__impl_get_node_type!(get_node_, KinematicBody2D);
__impl_get_node_type!(get_node_, Label);
__impl_get_node_type!(get_node_, Light2D);
__impl_get_node_type!(get_node_, LightOccluder2D);
__impl_get_node_type!(get_node_, Line2D);
__impl_get_node_type!(get_node_, LineEdit);
__impl_get_node_type!(get_node_, LinkButton);
__impl_get_node_type!(get_node_, Listener);
__impl_get_node_type!(get_node_, Listener2D);
__impl_get_node_type!(get_node_, MarginContainer);
__impl_get_node_type!(get_node_, MenuButton);
__impl_get_node_type!(get_node_, MeshInstance2D);
__impl_get_node_type!(get_node_, MultiMeshInstance2D);
__impl_get_node_type!(get_node_, Navigation);
__impl_get_node_type!(get_node_, Navigation2D);
__impl_get_node_type!(get_node_, NavigationMeshInstance);
__impl_get_node_type!(get_node_, NavigationPolygonInstance);
__impl_get_node_type!(get_node_, NinePatchRect);
__impl_get_node_type!(get_node_, Node2D);
__impl_get_node_type!(get_node_, Occluder);
__impl_get_node_type!(get_node_, OptionButton);
__impl_get_node_type!(get_node_, Panel);
__impl_get_node_type!(get_node_, PanelContainer);
__impl_get_node_type!(get_node_, ParallaxBackground);
__impl_get_node_type!(get_node_, ParallaxLayer);
__impl_get_node_type!(get_node_, Particles2D);
__impl_get_node_type!(get_node_, Path);
__impl_get_node_type!(get_node_, Path2D);
__impl_get_node_type!(get_node_, PathFollow);
__impl_get_node_type!(get_node_, PathFollow2D);
__impl_get_node_type!(get_node_, PhysicalBone);
__impl_get_node_type!(get_node_, PhysicsBody);
__impl_get_node_type!(get_node_, PhysicsBody2D);
__impl_get_node_type!(get_node_, PinJoint);
__impl_get_node_type!(get_node_, PinJoint2D);
__impl_get_node_type!(get_node_, Polygon2D);
__impl_get_node_type!(get_node_, Popup);
__impl_get_node_type!(get_node_, PopupDialog);
__impl_get_node_type!(get_node_, PopupMenu);
__impl_get_node_type!(get_node_, PopupPanel);
__impl_get_node_type!(get_node_, Portal);
__impl_get_node_type!(get_node_, Position2D);
__impl_get_node_type!(get_node_, Position3D);
__impl_get_node_type!(get_node_, ProgressBar);
__impl_get_node_type!(get_node_, ProximityGroup);
__impl_get_node_type!(get_node_, Range);
__impl_get_node_type!(get_node_, RayCast);
__impl_get_node_type!(get_node_, RayCast2D);
__impl_get_node_type!(get_node_, ReferenceRect);
__impl_get_node_type!(get_node_, RemoteTransform);
__impl_get_node_type!(get_node_, RemoteTransform2D);
__impl_get_node_type!(get_node_, ResourcePreloader);
__impl_get_node_type!(get_node_, RichTextLabel);
__impl_get_node_type!(get_node_, RigidBody);
__impl_get_node_type!(get_node_, RigidBody2D);
__impl_get_node_type!(get_node_, Room);
__impl_get_node_type!(get_node_, RoomGroup);
__impl_get_node_type!(get_node_, RoomManager);
__impl_get_node_type!(get_node_, ScriptCreateDialog);
__impl_get_node_type!(get_node_, ScriptEditor);
__impl_get_node_type!(get_node_, ScrollBar);
__impl_get_node_type!(get_node_, ScrollContainer);
__impl_get_node_type!(get_node_, Separator);
__impl_get_node_type!(get_node_, Skeleton);
__impl_get_node_type!(get_node_, Skeleton2D);
__impl_get_node_type!(get_node_, SkeletonIK);
__impl_get_node_type!(get_node_, Slider);
__impl_get_node_type!(get_node_, SliderJoint);
__impl_get_node_type!(get_node_, Spatial);
__impl_get_node_type!(get_node_, SpinBox);
__impl_get_node_type!(get_node_, SplitContainer);
__impl_get_node_type!(get_node_, SpringArm);
__impl_get_node_type!(get_node_, Sprite);
__impl_get_node_type!(get_node_, StaticBody);
__impl_get_node_type!(get_node_, StaticBody2D);
__impl_get_node_type!(get_node_, TabContainer);
__impl_get_node_type!(get_node_, Tabs);
__impl_get_node_type!(get_node_, TextEdit);
__impl_get_node_type!(get_node_, TextureButton);
__impl_get_node_type!(get_node_, TextureProgress);
__impl_get_node_type!(get_node_, TextureRect);
__impl_get_node_type!(get_node_, TileMap);
__impl_get_node_type!(get_node_, Timer);
__impl_get_node_type!(get_node_, ToolButton);
__impl_get_node_type!(get_node_, TouchScreenButton);
__impl_get_node_type!(get_node_, Tree);
__impl_get_node_type!(get_node_, Tween);
__impl_get_node_type!(get_node_, VBoxContainer);
__impl_get_node_type!(get_node_, VehicleBody);
__impl_get_node_type!(get_node_, VehicleWheel);
__impl_get_node_type!(get_node_, VideoPlayer);
__impl_get_node_type!(get_node_, Viewport);
__impl_get_node_type!(get_node_, ViewportContainer);
__impl_get_node_type!(get_node_, VisibilityEnabler2D);
__impl_get_node_type!(get_node_, VisibilityNotifier2D);
__impl_get_node_type!(get_node_, VScrollBar);
__impl_get_node_type!(get_node_, VSeparator);
__impl_get_node_type!(get_node_, VSlider);
__impl_get_node_type!(get_node_, VSplitContainer);
__impl_get_node_type!(get_node_, WindowDialog);
__impl_get_node_type!(get_node_, WorldEnvironment);
__impl_get_node_type!(get_node_, YSort);
