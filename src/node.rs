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
__impl_get_node_type!(get_node_arvr_camera, ARVRCamera);
__impl_get_node_type!(get_node_arvr_controller, ARVRController);
__impl_get_node_type!(get_node_arvr_arvr_origin, ARVROrigin);
__impl_get_node_type!(get_node_aspect_ratio_container, AspectRatioContainer);
__impl_get_node_type!(get_node_audio_stream_player, AudioStreamPlayer);
__impl_get_node_type!(get_node_audio_stream_player_2d, AudioStreamPlayer2D);
__impl_get_node_type!(get_node_audio_stream_player_3d, AudioStreamPlayer3D);
__impl_get_node_type!(get_node_back_buffer_copy, BackBufferCopy);
__impl_get_node_type!(get_node_base_button, BaseButton);
__impl_get_node_type!(get_node_bone_2d, Bone2D);
__impl_get_node_type!(get_node_bone_attachment, BoneAttachment);
__impl_get_node_type!(get_node_box_container, BoxContainer);
__impl_get_node_type!(get_node_button, Button);
__impl_get_node_type!(get_node_camera, Camera);
__impl_get_node_type!(get_node_camera_2d, Camera2D);
__impl_get_node_type!(get_node_canvas_item, CanvasItem);
__impl_get_node_type!(get_node_canvas_layer, CanvasLayer);
__impl_get_node_type!(get_node_canvas_modulate, CanvasModulate);
__impl_get_node_type!(get_node_center_container, CenterContainer);
__impl_get_node_type!(get_node_check_box, CheckBox);
__impl_get_node_type!(get_node_check_button, CheckButton);
__impl_get_node_type!(get_node_clipped_camera, ClippedCamera);
__impl_get_node_type!(get_node_collision_object, CollisionObject);
__impl_get_node_type!(get_node_collision_object_2d, CollisionObject2D);
__impl_get_node_type!(get_node_collision_polygon, CollisionPolygon);
__impl_get_node_type!(get_node_collision_polygon_2d, CollisionPolygon2D);
__impl_get_node_type!(get_node_collision_shape, CollisionShape);
__impl_get_node_type!(get_node_collision_shape_2d, CollisionShape2D);
__impl_get_node_type!(get_node_color_picker, ColorPicker);
__impl_get_node_type!(get_node_color_picker_button, ColorPickerButton);
__impl_get_node_type!(get_node_color_rect, ColorRect);
__impl_get_node_type!(get_node_cone_twist_joint, ConeTwistJoint);
__impl_get_node_type!(get_node_confirmation_dialog, ConfirmationDialog);
__impl_get_node_type!(get_node_container, Container);
__impl_get_node_type!(get_node_control, Control);
__impl_get_node_type!(get_node_cpu_particles_2d, CPUParticles2D);
__impl_get_node_type!(get_node_cull_instance, CullInstance);
__impl_get_node_type!(get_node_damped_spring_joint_2d, DampedSpringJoint2D);
__impl_get_node_type!(get_node_editor_file_dialog, EditorFileDialog);
__impl_get_node_type!(get_node_editor_file_system, EditorFileSystem);
__impl_get_node_type!(get_node_editor_inspector, EditorInspector);
__impl_get_node_type!(get_node_editor_interface, EditorInterface);
__impl_get_node_type!(get_node_editor_plugin, EditorPlugin);
__impl_get_node_type!(get_node_editor_property, EditorProperty);
__impl_get_node_type!(get_node_editor_resource_preview, EditorResourcePreview);
__impl_get_node_type!(get_node_editor_script_picker, EditorScriptPicker);
__impl_get_node_type!(get_node_editor_spin_slider, EditorSpinSlider);
__impl_get_node_type!(get_node_file_dialog, FileDialog);
__impl_get_node_type!(get_node_file_system_dock, FileSystemDock);
__impl_get_node_type!(get_node_generic_6dof_joint, Generic6DOFJoint);
__impl_get_node_type!(get_node_graph_edit, GraphEdit);
__impl_get_node_type!(get_node_graph_node, GraphNode);
__impl_get_node_type!(get_node_grid_container, GridContainer);
__impl_get_node_type!(get_node_grid_map, GridMap);
__impl_get_node_type!(get_node_groove_joint_2d, GrooveJoint2D);
__impl_get_node_type!(get_node_h_box_container, HBoxContainer);
__impl_get_node_type!(get_node_hinge_joint, HingeJoint);
__impl_get_node_type!(get_node_h_scroll_bar, HScrollBar);
__impl_get_node_type!(get_node_h_separator, HSeparator);
__impl_get_node_type!(get_node_h_slider, HSlider);
__impl_get_node_type!(get_node_h_split_container, HSplitContainer);
__impl_get_node_type!(get_node_http_request, HTTPRequest);
__impl_get_node_type!(get_node_instance_placeholder, InstancePlaceholder);
__impl_get_node_type!(get_node_interpolated_camera, InterpolatedCamera);
__impl_get_node_type!(get_node_item_list, ItemList);
__impl_get_node_type!(get_node_joint, Joint);
__impl_get_node_type!(get_node_joint_2d, Joint2D);
__impl_get_node_type!(get_node_kinematic_body, KinematicBody);
__impl_get_node_type!(get_node_kinematic_body_2d, KinematicBody2D);
__impl_get_node_type!(get_node_label, Label);
__impl_get_node_type!(get_node_light_2d, Light2D);
__impl_get_node_type!(get_node_light_occluder_2d, LightOccluder2D);
__impl_get_node_type!(get_node_line_2d, Line2D);
__impl_get_node_type!(get_node_line_edit, LineEdit);
__impl_get_node_type!(get_node_link_button, LinkButton);
__impl_get_node_type!(get_node_listener, Listener);
__impl_get_node_type!(get_node_listener_2d, Listener2D);
__impl_get_node_type!(get_node_margin_container, MarginContainer);
__impl_get_node_type!(get_node_menu_button, MenuButton);
__impl_get_node_type!(get_node_mesh_instance_2d, MeshInstance2D);
__impl_get_node_type!(get_node_multi_mesh_instance_2d, MultiMeshInstance2D);
__impl_get_node_type!(get_node_navigation, Navigation);
__impl_get_node_type!(get_node_navigation_2d, Navigation2D);
__impl_get_node_type!(get_node_navigation_mesh_instance, NavigationMeshInstance);
__impl_get_node_type!(
    get_node_navigation_polygon_instance,
    NavigationPolygonInstance
);
__impl_get_node_type!(get_node_nine_patch_rect, NinePatchRect);
__impl_get_node_type!(get_node_node_2d, Node2D);
__impl_get_node_type!(get_node_occluder, Occluder);
__impl_get_node_type!(get_node_option_button, OptionButton);
__impl_get_node_type!(get_node_panel, Panel);
__impl_get_node_type!(get_node_panel_container, PanelContainer);
__impl_get_node_type!(get_node_parallax_background, ParallaxBackground);
__impl_get_node_type!(get_node_parallax_layer, ParallaxLayer);
__impl_get_node_type!(get_node_particles_2d, Particles2D);
__impl_get_node_type!(get_node_path, Path);
__impl_get_node_type!(get_node_path_2d, Path2D);
__impl_get_node_type!(get_node_path_follow, PathFollow);
__impl_get_node_type!(get_node_path_follow_2d, PathFollow2D);
__impl_get_node_type!(get_node_physical_bone, PhysicalBone);
__impl_get_node_type!(get_node_physics_body, PhysicsBody);
__impl_get_node_type!(get_node_physics_body_2d, PhysicsBody2D);
__impl_get_node_type!(get_node_pin_joint, PinJoint);
__impl_get_node_type!(get_node_pin_joint_2d, PinJoint2D);
__impl_get_node_type!(get_node_polygon_2d, Polygon2D);
__impl_get_node_type!(get_node_popup, Popup);
__impl_get_node_type!(get_node_popup_dialog, PopupDialog);
__impl_get_node_type!(get_node_popup_menu, PopupMenu);
__impl_get_node_type!(get_node_popup_panel, PopupPanel);
__impl_get_node_type!(get_node_portal, Portal);
__impl_get_node_type!(get_node_position_2d, Position2D);
__impl_get_node_type!(get_node_position_3d, Position3D);
__impl_get_node_type!(get_node_progress_bar, ProgressBar);
__impl_get_node_type!(get_node_proximity_group, ProximityGroup);
__impl_get_node_type!(get_node_range, Range);
__impl_get_node_type!(get_node_ray_cast, RayCast);
__impl_get_node_type!(get_node_ray_cast_2d, RayCast2D);
__impl_get_node_type!(get_node_reference_rect, ReferenceRect);
__impl_get_node_type!(get_node_remote_transform, RemoteTransform);
__impl_get_node_type!(get_node_remote_transform_2d, RemoteTransform2D);
__impl_get_node_type!(get_node_resource_preloader, ResourcePreloader);
__impl_get_node_type!(get_node_rich_text_label, RichTextLabel);
__impl_get_node_type!(get_node_rigid_body, RigidBody);
__impl_get_node_type!(get_node_rigid_body_2d, RigidBody2D);
__impl_get_node_type!(get_node_room, Room);
__impl_get_node_type!(get_node_room_group, RoomGroup);
__impl_get_node_type!(get_node_room_manager, RoomManager);
__impl_get_node_type!(get_node_script_create_dialog, ScriptCreateDialog);
__impl_get_node_type!(get_node_script_editor, ScriptEditor);
__impl_get_node_type!(get_node_scroll_bar, ScrollBar);
__impl_get_node_type!(get_node_scroll_container, ScrollContainer);
__impl_get_node_type!(get_node_separator, Separator);
__impl_get_node_type!(get_node_skeleton, Skeleton);
__impl_get_node_type!(get_node_skeleton_2d, Skeleton2D);
__impl_get_node_type!(get_node_skeleton_ik, SkeletonIK);
__impl_get_node_type!(get_node_slider, Slider);
__impl_get_node_type!(get_node_slider_joint, SliderJoint);
__impl_get_node_type!(get_node_spatial, Spatial);
__impl_get_node_type!(get_node_spin_box, SpinBox);
__impl_get_node_type!(get_node_split_container, SplitContainer);
__impl_get_node_type!(get_node_spring_arm, SpringArm);
__impl_get_node_type!(get_node_sprite, Sprite);
__impl_get_node_type!(get_node_static_body, StaticBody);
__impl_get_node_type!(get_node_static_body_2d, StaticBody2D);
__impl_get_node_type!(get_node_tab_container, TabContainer);
__impl_get_node_type!(get_node_tabs, Tabs);
__impl_get_node_type!(get_node_text_edit, TextEdit);
__impl_get_node_type!(get_node_texture_button, TextureButton);
__impl_get_node_type!(get_node_texture_progress, TextureProgress);
__impl_get_node_type!(get_node_texture_rect, TextureRect);
__impl_get_node_type!(get_node_tile_map, TileMap);
__impl_get_node_type!(get_node_timer, Timer);
__impl_get_node_type!(get_node_tool_button, ToolButton);
__impl_get_node_type!(get_node_touch_screen_button, TouchScreenButton);
__impl_get_node_type!(get_node_tree, Tree);
__impl_get_node_type!(get_node_tween, Tween);
__impl_get_node_type!(get_node_v_box_container, VBoxContainer);
__impl_get_node_type!(get_node_vehicle_body, VehicleBody);
__impl_get_node_type!(get_node_vehicle_wheel, VehicleWheel);
__impl_get_node_type!(get_node_video_player, VideoPlayer);
__impl_get_node_type!(get_node_viewport, Viewport);
__impl_get_node_type!(get_node_viewport_container, ViewportContainer);
__impl_get_node_type!(get_node_visibility_enabler_2d, VisibilityEnabler2D);
__impl_get_node_type!(get_node_visibility_notifier_2d, VisibilityNotifier2D);
__impl_get_node_type!(get_node_v_scroll_bar, VScrollBar);
__impl_get_node_type!(get_node_v_separator, VSeparator);
__impl_get_node_type!(get_node_v_slider, VSlider);
__impl_get_node_type!(get_node_v_split_container, VSplitContainer);
__impl_get_node_type!(get_node_window_dialog, WindowDialog);
__impl_get_node_type!(get_node_world_environment, WorldEnvironment);
__impl_get_node_type!(get_node_y_sort, YSort);

#[cfg(test)]
mod test_compilation {
    use gdnative::api::*;
    use gdnative::object::TRef;

    #[test]
    fn test_get_tree() {
        let node = Node::new();
        let _: TRef<SceneTree> = get_tree!(node);
    }

    #[test]
    fn test_get_root() {
        let node = Node::new();
        let _: TRef<Viewport> = get_root!(node);
    }

    #[test]
    fn test_get_node() {
        let node = Node::new();
        let _: TRef<Node> = get_node!(node, "");
    }

    #[macro_export]
    macro_rules! __impl_get_node_type_test {
        ($name:ident, $type:tt) => {
            paste::item! {
                #[test]
                fn [<test_ $name>]() {
                    let node = Node::new();
                    let _: TRef<$type> = $name!(node, "");
                }
            }
        };
    }

    __impl_get_node_type_test!(get_node_accept_dialog, AcceptDialog);
    __impl_get_node_type_test!(get_node_animated_sprite, AnimatedSprite);
    __impl_get_node_type_test!(get_node_animation_player, AnimationPlayer);
    __impl_get_node_type_test!(get_node_animation_tree, AnimationTree);
    __impl_get_node_type_test!(get_node_animation_tree_player, AnimationTreePlayer);
    __impl_get_node_type_test!(get_node_area, Area);
    __impl_get_node_type_test!(get_node_area_2d, Area2D);
    __impl_get_node_type_test!(get_node_arvr_anchor, ARVRAnchor);
    __impl_get_node_type_test!(get_node_arvr_camera, ARVRCamera);
    __impl_get_node_type_test!(get_node_arvr_controller, ARVRController);
    __impl_get_node_type_test!(get_node_arvr_arvr_origin, ARVROrigin);
    __impl_get_node_type_test!(get_node_aspect_ratio_container, AspectRatioContainer);
    __impl_get_node_type_test!(get_node_audio_stream_player, AudioStreamPlayer);
    __impl_get_node_type_test!(get_node_audio_stream_player_2d, AudioStreamPlayer2D);
    __impl_get_node_type_test!(get_node_audio_stream_player_3d, AudioStreamPlayer3D);
    __impl_get_node_type_test!(get_node_back_buffer_copy, BackBufferCopy);
    __impl_get_node_type_test!(get_node_base_button, BaseButton);
    __impl_get_node_type_test!(get_node_bone_2d, Bone2D);
    __impl_get_node_type_test!(get_node_bone_attachment, BoneAttachment);
    __impl_get_node_type_test!(get_node_box_container, BoxContainer);
    __impl_get_node_type_test!(get_node_button, Button);
    __impl_get_node_type_test!(get_node_camera, Camera);
    __impl_get_node_type_test!(get_node_camera_2d, Camera2D);
    __impl_get_node_type_test!(get_node_canvas_item, CanvasItem);
    __impl_get_node_type_test!(get_node_canvas_layer, CanvasLayer);
    __impl_get_node_type_test!(get_node_canvas_modulate, CanvasModulate);
    __impl_get_node_type_test!(get_node_center_container, CenterContainer);
    __impl_get_node_type_test!(get_node_check_box, CheckBox);
    __impl_get_node_type_test!(get_node_check_button, CheckButton);
    __impl_get_node_type_test!(get_node_clipped_camera, ClippedCamera);
    __impl_get_node_type_test!(get_node_collision_object, CollisionObject);
    __impl_get_node_type_test!(get_node_collision_object_2d, CollisionObject2D);
    __impl_get_node_type_test!(get_node_collision_polygon, CollisionPolygon);
    __impl_get_node_type_test!(get_node_collision_polygon_2d, CollisionPolygon2D);
    __impl_get_node_type_test!(get_node_collision_shape, CollisionShape);
    __impl_get_node_type_test!(get_node_collision_shape_2d, CollisionShape2D);
    __impl_get_node_type_test!(get_node_color_picker, ColorPicker);
    __impl_get_node_type_test!(get_node_color_picker_button, ColorPickerButton);
    __impl_get_node_type_test!(get_node_color_rect, ColorRect);
    __impl_get_node_type_test!(get_node_cone_twist_joint, ConeTwistJoint);
    __impl_get_node_type_test!(get_node_confirmation_dialog, ConfirmationDialog);
    __impl_get_node_type_test!(get_node_container, Container);
    __impl_get_node_type_test!(get_node_control, Control);
    __impl_get_node_type_test!(get_node_cpu_particles_2d, CPUParticles2D);
    __impl_get_node_type_test!(get_node_cull_instance, CullInstance);
    __impl_get_node_type_test!(get_node_damped_spring_joint_2d, DampedSpringJoint2D);
    __impl_get_node_type_test!(get_node_editor_file_dialog, EditorFileDialog);
    __impl_get_node_type_test!(get_node_editor_file_system, EditorFileSystem);
    __impl_get_node_type_test!(get_node_editor_inspector, EditorInspector);
    __impl_get_node_type_test!(get_node_editor_interface, EditorInterface);
    __impl_get_node_type_test!(get_node_editor_plugin, EditorPlugin);
    __impl_get_node_type_test!(get_node_editor_property, EditorProperty);
    __impl_get_node_type_test!(get_node_editor_resource_preview, EditorResourcePreview);
    __impl_get_node_type_test!(get_node_editor_script_picker, EditorScriptPicker);
    __impl_get_node_type_test!(get_node_editor_spin_slider, EditorSpinSlider);
    __impl_get_node_type_test!(get_node_file_dialog, FileDialog);
    __impl_get_node_type_test!(get_node_file_system_dock, FileSystemDock);
    __impl_get_node_type_test!(get_node_generic_6dof_joint, Generic6DOFJoint);
    __impl_get_node_type_test!(get_node_graph_edit, GraphEdit);
    __impl_get_node_type_test!(get_node_graph_node, GraphNode);
    __impl_get_node_type_test!(get_node_grid_container, GridContainer);
    __impl_get_node_type_test!(get_node_grid_map, GridMap);
    __impl_get_node_type_test!(get_node_groove_joint_2d, GrooveJoint2D);
    __impl_get_node_type_test!(get_node_h_box_container, HBoxContainer);
    __impl_get_node_type_test!(get_node_hinge_joint, HingeJoint);
    __impl_get_node_type_test!(get_node_h_scroll_bar, HScrollBar);
    __impl_get_node_type_test!(get_node_h_separator, HSeparator);
    __impl_get_node_type_test!(get_node_h_slider, HSlider);
    __impl_get_node_type_test!(get_node_h_split_container, HSplitContainer);
    __impl_get_node_type_test!(get_node_http_request, HTTPRequest);
    __impl_get_node_type_test!(get_node_instance_placeholder, InstancePlaceholder);
    __impl_get_node_type_test!(get_node_interpolated_camera, InterpolatedCamera);
    __impl_get_node_type_test!(get_node_item_list, ItemList);
    __impl_get_node_type_test!(get_node_joint, Joint);
    __impl_get_node_type_test!(get_node_joint_2d, Joint2D);
    __impl_get_node_type_test!(get_node_kinematic_body, KinematicBody);
    __impl_get_node_type_test!(get_node_kinematic_body_2d, KinematicBody2D);
    __impl_get_node_type_test!(get_node_label, Label);
    __impl_get_node_type_test!(get_node_light_2d, Light2D);
    __impl_get_node_type_test!(get_node_light_occluder_2d, LightOccluder2D);
    __impl_get_node_type_test!(get_node_line_2d, Line2D);
    __impl_get_node_type_test!(get_node_line_edit, LineEdit);
    __impl_get_node_type_test!(get_node_link_button, LinkButton);
    __impl_get_node_type_test!(get_node_listener, Listener);
    __impl_get_node_type_test!(get_node_listener_2d, Listener2D);
    __impl_get_node_type_test!(get_node_margin_container, MarginContainer);
    __impl_get_node_type_test!(get_node_menu_button, MenuButton);
    __impl_get_node_type_test!(get_node_mesh_instance_2d, MeshInstance2D);
    __impl_get_node_type_test!(get_node_multi_mesh_instance_2d, MultiMeshInstance2D);
    __impl_get_node_type_test!(get_node_navigation, Navigation);
    __impl_get_node_type_test!(get_node_navigation_2d, Navigation2D);
    __impl_get_node_type_test!(get_node_navigation_mesh_instance, NavigationMeshInstance);
    __impl_get_node_type_test!(
        get_node_navigation_polygon_instance,
        NavigationPolygonInstance
    );
    __impl_get_node_type_test!(get_node_nine_patch_rect, NinePatchRect);
    __impl_get_node_type_test!(get_node_node_2d, Node2D);
    __impl_get_node_type_test!(get_node_occluder, Occluder);
    __impl_get_node_type_test!(get_node_option_button, OptionButton);
    __impl_get_node_type_test!(get_node_panel, Panel);
    __impl_get_node_type_test!(get_node_panel_container, PanelContainer);
    __impl_get_node_type_test!(get_node_parallax_background, ParallaxBackground);
    __impl_get_node_type_test!(get_node_parallax_layer, ParallaxLayer);
    __impl_get_node_type_test!(get_node_particles_2d, Particles2D);
    __impl_get_node_type_test!(get_node_path, Path);
    __impl_get_node_type_test!(get_node_path_2d, Path2D);
    __impl_get_node_type_test!(get_node_path_follow, PathFollow);
    __impl_get_node_type_test!(get_node_path_follow_2d, PathFollow2D);
    __impl_get_node_type_test!(get_node_physical_bone, PhysicalBone);
    __impl_get_node_type_test!(get_node_physics_body, PhysicsBody);
    __impl_get_node_type_test!(get_node_physics_body_2d, PhysicsBody2D);
    __impl_get_node_type_test!(get_node_pin_joint, PinJoint);
    __impl_get_node_type_test!(get_node_pin_joint_2d, PinJoint2D);
    __impl_get_node_type_test!(get_node_polygon_2d, Polygon2D);
    __impl_get_node_type_test!(get_node_popup, Popup);
    __impl_get_node_type_test!(get_node_popup_dialog, PopupDialog);
    __impl_get_node_type_test!(get_node_popup_menu, PopupMenu);
    __impl_get_node_type_test!(get_node_popup_panel, PopupPanel);
    __impl_get_node_type_test!(get_node_portal, Portal);
    __impl_get_node_type_test!(get_node_position_2d, Position2D);
    __impl_get_node_type_test!(get_node_position_3d, Position3D);
    __impl_get_node_type_test!(get_node_progress_bar, ProgressBar);
    __impl_get_node_type_test!(get_node_proximity_group, ProximityGroup);
    __impl_get_node_type_test!(get_node_range, Range);
    __impl_get_node_type_test!(get_node_ray_cast, RayCast);
    __impl_get_node_type_test!(get_node_ray_cast_2d, RayCast2D);
    __impl_get_node_type_test!(get_node_reference_rect, ReferenceRect);
    __impl_get_node_type_test!(get_node_remote_transform, RemoteTransform);
    __impl_get_node_type_test!(get_node_remote_transform_2d, RemoteTransform2D);
    __impl_get_node_type_test!(get_node_resource_preloader, ResourcePreloader);
    __impl_get_node_type_test!(get_node_rich_text_label, RichTextLabel);
    __impl_get_node_type_test!(get_node_rigid_body, RigidBody);
    __impl_get_node_type_test!(get_node_rigid_body_2d, RigidBody2D);
    __impl_get_node_type_test!(get_node_room, Room);
    __impl_get_node_type_test!(get_node_room_group, RoomGroup);
    __impl_get_node_type_test!(get_node_room_manager, RoomManager);
    __impl_get_node_type_test!(get_node_script_create_dialog, ScriptCreateDialog);
    __impl_get_node_type_test!(get_node_script_editor, ScriptEditor);
    __impl_get_node_type_test!(get_node_scroll_bar, ScrollBar);
    __impl_get_node_type_test!(get_node_scroll_container, ScrollContainer);
    __impl_get_node_type_test!(get_node_separator, Separator);
    __impl_get_node_type_test!(get_node_skeleton, Skeleton);
    __impl_get_node_type_test!(get_node_skeleton_2d, Skeleton2D);
    __impl_get_node_type_test!(get_node_skeleton_ik, SkeletonIK);
    __impl_get_node_type_test!(get_node_slider, Slider);
    __impl_get_node_type_test!(get_node_slider_joint, SliderJoint);
    __impl_get_node_type_test!(get_node_spatial, Spatial);
    __impl_get_node_type_test!(get_node_spin_box, SpinBox);
    __impl_get_node_type_test!(get_node_split_container, SplitContainer);
    __impl_get_node_type_test!(get_node_spring_arm, SpringArm);
    __impl_get_node_type_test!(get_node_sprite, Sprite);
    __impl_get_node_type_test!(get_node_static_body, StaticBody);
    __impl_get_node_type_test!(get_node_static_body_2d, StaticBody2D);
    __impl_get_node_type_test!(get_node_tab_container, TabContainer);
    __impl_get_node_type_test!(get_node_tabs, Tabs);
    __impl_get_node_type_test!(get_node_text_edit, TextEdit);
    __impl_get_node_type_test!(get_node_texture_button, TextureButton);
    __impl_get_node_type_test!(get_node_texture_progress, TextureProgress);
    __impl_get_node_type_test!(get_node_texture_rect, TextureRect);
    __impl_get_node_type_test!(get_node_tile_map, TileMap);
    __impl_get_node_type_test!(get_node_timer, Timer);
    __impl_get_node_type_test!(get_node_tool_button, ToolButton);
    __impl_get_node_type_test!(get_node_touch_screen_button, TouchScreenButton);
    __impl_get_node_type_test!(get_node_tree, Tree);
    __impl_get_node_type_test!(get_node_tween, Tween);
    __impl_get_node_type_test!(get_node_v_box_container, VBoxContainer);
    __impl_get_node_type_test!(get_node_vehicle_body, VehicleBody);
    __impl_get_node_type_test!(get_node_vehicle_wheel, VehicleWheel);
    __impl_get_node_type_test!(get_node_video_player, VideoPlayer);
    __impl_get_node_type_test!(get_node_viewport, Viewport);
    __impl_get_node_type_test!(get_node_viewport_container, ViewportContainer);
    __impl_get_node_type_test!(get_node_visibility_enabler_2d, VisibilityEnabler2D);
    __impl_get_node_type_test!(get_node_visibility_notifier_2d, VisibilityNotifier2D);
    __impl_get_node_type_test!(get_node_v_scroll_bar, VScrollBar);
    __impl_get_node_type_test!(get_node_v_separator, VSeparator);
    __impl_get_node_type_test!(get_node_v_slider, VSlider);
    __impl_get_node_type_test!(get_node_v_split_container, VSplitContainer);
    __impl_get_node_type_test!(get_node_window_dialog, WindowDialog);
    __impl_get_node_type_test!(get_node_world_environment, WorldEnvironment);
    __impl_get_node_type_test!(get_node_y_sort, YSort);
}
