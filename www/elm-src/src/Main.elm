module Main exposing (..)

import Browser
import Html
import Html.Attributes as HtmlAttr
import Html.Events as HtmlEv



-- MAIN


main =
    Browser.sandbox { init = init, update = update, view = view }



-- MODEL


type alias Model =
    { selected : List String }


init : Model
init =
    { selected = [ "coll-a", "coll-b", "coll-c" ] }



-- UPDATE


type Msg
    = Check String
    | Uncheck String


update : Msg -> Model -> Model
update msg model =
    case msg of
        Uncheck s ->
            { model | selected = List.filter (\s_ -> s_ /= s) model.selected }

        Check s ->
            { model | selected = s :: model.selected }



-- VIEW


view : Model -> Html.Html Msg
view model =
    let
        selector =
            Html.div
                [ HtmlAttr.style "flex" "0.2"
                , HtmlAttr.style "padding" "8px"
                ]
            <|
                List.map
                    (\name ->
                        Html.div []
                            [ Html.input
                                [ HtmlAttr.value name
                                , HtmlAttr.type_ "checkbox"
                                , HtmlAttr.checked <|
                                    not <|
                                        List.isEmpty <|
                                            List.filter ((==) name) model.selected
                                , HtmlEv.onCheck
                                    (\checked ->
                                        if checked then
                                            Check name

                                        else
                                            Uncheck name
                                    )
                                ]
                                []
                            , Html.label [] [ Html.text name ]
                            ]
                    )
                    [ "coll-a", "coll-b", "coll-c" ]
    in
    Html.div
        [ HtmlAttr.style "display" "flex"
        ]
        [ selector
        , Html.node "wasm-wrapper"
            (List.map (\name -> HtmlAttr.attribute name "true") model.selected)
            []
        ]
